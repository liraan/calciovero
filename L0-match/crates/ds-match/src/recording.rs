//! The recorded match: the tick record as a public, versioned contract
//! (ADR-065, decision U).
//!
//! **One responsibility:** pack the tick record of a finished match with
//! its header (the format version, the engine that played it, the physics
//! it was played with) and unpack it exactly. The views (2D, 2.5D, 3D)
//! depend on this format, never on the engine.
//!
//! - **Its own version.** [`RECORD_FORMAT_VERSION`] is not `SIM_VERSION`:
//!   the engine can change how football is played without changing what a
//!   record looks like, and a record keeps the version it was written with.
//! - **New fields are optional.** A field added to the record is a new
//!   column at the end, read only from records whose format has it; an
//!   older record gives it its default, and the views draw their fallback.
//!   A field is never removed nor its meaning changed (that would be a new
//!   format, with its migration).
//! - **The physics in the header.** The ball's gravity and friction, the
//!   tick rate and the pitch: a view flies the ball between two ticks with
//!   the numbers the match was played with, not the current ones.
//! - **Packed.** Column by column (every field of every tick together),
//!   each value as the difference from its prediction (the last value; for
//!   positions, the last value plus the last step), zig-zag varints, then
//!   deflate: about 2.3 MB a match instead of 130 MB of JSON.
//!
//! The engine is never changed for a view: a new field is born from a
//! change of football with its ADR and its `SIM_VERSION`, then the record
//! carries it and the view shows it.

use ds_core::ids::PlayerId;
use ds_stats::Half;
use serde::{Deserialize, Serialize};

use crate::balance::BallBalance;
use crate::record::{AgentFrame, BallFrame, TickRecord};
use crate::result::Score;
use crate::spatial::BallState;

/// The current version of the record format.
///
/// v1 (2026-11-16, ADR-065): the fields of `TickRecord` at SIM 137.
pub const RECORD_FORMAT_VERSION: u32 = 1;

/// The physics a match was played with, for the views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordPhysics {
    /// Ticks per second.
    pub ticks_per_second: u8,
    /// The ball: gravity, friction, rebound (`balance/ball.json`).
    pub ball: BallBalance,
    /// The pitch, metres, `(length, width)`.
    pub pitch_m: (u16, u16),
}

/// The header of a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordHeader {
    /// The format the record is written in.
    pub format_version: u32,
    /// The engine that played the match.
    pub sim_version: u32,
    /// The physics it was played with.
    pub physics: RecordPhysics,
}

impl RecordHeader {
    /// The header of a match played now, by this engine.
    #[must_use]
    pub fn current(ticks_per_second: u8, pitch_m: (u16, u16)) -> Self {
        Self {
            format_version: RECORD_FORMAT_VERSION,
            sim_version: crate::save::SIM_VERSION,
            physics: RecordPhysics {
                ticks_per_second,
                ball: BallBalance::load(),
                pitch_m,
            },
        }
    }
}

/// A recorded match, packed: the header in clear, the ticks compressed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackedRecord {
    /// The header.
    pub header: RecordHeader,
    /// How many ticks.
    pub ticks: u32,
    /// The columns, deflated, in base64.
    pub data: String,
}

/// Why a record cannot be read.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RecordError {
    /// Written by a newer format than this build knows.
    #[error("record format {record} is newer than this build's {current}")]
    FormatTooNew {
        /// The record's format.
        record: u32,
        /// This build's.
        current: u32,
    },
    /// The data is damaged.
    #[error("the record's data is damaged")]
    Damaged,
}

// ---- the columns ----------------------------------------------------------

const fn zigzag(v: i64) -> u64 {
    #[allow(clippy::cast_sign_loss)]
    let z = ((v << 1) ^ (v >> 63)) as u64;
    z
}

const fn unzigzag(z: u64) -> i64 {
    #[allow(clippy::cast_possible_wrap)]
    let v = ((z >> 1) as i64) ^ -((z & 1) as i64);
    v
}

#[derive(Default)]
struct Writer(Vec<u8>);

impl Writer {
    fn uint(&mut self, mut v: u64) {
        loop {
            #[allow(clippy::cast_possible_truncation)]
            let b = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                self.0.push(b);
                return;
            }
            self.0.push(b | 0x80);
        }
    }
    fn int(&mut self, v: i64) {
        self.uint(zigzag(v));
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn uint(&mut self) -> Result<u64, RecordError> {
        let mut v = 0_u64;
        for (i, shift) in (0..64).step_by(7).enumerate() {
            let b = *self.bytes.get(self.at).ok_or(RecordError::Damaged)?;
            self.at += 1;
            // The last byte of a varint may only contribute its low bit
            // (V-03): more than that is a non-canonical, over-long encoding.
            if i == 9 && (b & 0x7e) != 0 {
                return Err(RecordError::Damaged);
            }
            // Overflow check on the accumulation (V-03).
            let addend = u64::from(b & 0x7f)
                .checked_shl(u32::try_from(shift).unwrap_or(0))
                .ok_or(RecordError::Damaged)?;
            v = v.checked_add(addend).ok_or(RecordError::Damaged)?;
            if b & 0x80 == 0 {
                return Ok(v);
            }
        }
        Err(RecordError::Damaged)
    }
    fn int(&mut self) -> Result<i64, RecordError> {
        self.uint().map(unzigzag)
    }
}

/// A column of values, each written as its difference from the prediction:
/// the last value (`order` 1) or the last value plus the last step
/// (`order` 2, positions).
fn put(w: &mut Writer, values: impl Iterator<Item = (i64, i64, i64)>) {
    for (v, p1, p2) in values {
        w.int(v - p1 - p2);
    }
}

/// The value in a column, from its two predecessors.
fn get(r: &mut Reader<'_>, p1: i64, p2: i64) -> Result<i64, RecordError> {
    r.int()?
        .checked_add(p1)
        .and_then(|v| v.checked_add(p2))
        .ok_or(RecordError::Damaged)
}

const fn half_code(h: Half) -> i64 {
    match h {
        Half::First => 0,
        Half::Second => 1,
        Half::ExtraFirst => 2,
        Half::ExtraSecond => 3,
        Half::Shootout => 4,
    }
}

const fn half_of(c: i64) -> Result<Half, RecordError> {
    Ok(match c {
        0 => Half::First,
        1 => Half::Second,
        2 => Half::ExtraFirst,
        3 => Half::ExtraSecond,
        4 => Half::Shootout,
        _ => return Err(RecordError::Damaged),
    })
}

const fn state_code(s: BallState) -> i64 {
    match s {
        BallState::Possessed => 0,
        BallState::InFlight => 1,
        BallState::Loose => 2,
    }
}

const fn state_of(c: i64) -> Result<BallState, RecordError> {
    Ok(match c {
        0 => BallState::Possessed,
        1 => BallState::InFlight,
        2 => BallState::Loose,
        _ => return Err(RecordError::Damaged),
    })
}

/// A player id as one number: `0` none, else `slot + 1`, the generation
/// in its own column.
fn id_code(id: Option<PlayerId>) -> (i64, i64) {
    id.map_or((0, 0), |p| {
        (i64::from(p.slot()) + 1, i64::from(p.generation()))
    })
}

fn id_of(slot: i64, generation: i64) -> Result<Option<PlayerId>, RecordError> {
    if slot == 0 {
        return Ok(None);
    }
    let s = u32::try_from(slot - 1).map_err(|_| RecordError::Damaged)?;
    let g = u32::try_from(generation).map_err(|_| RecordError::Damaged)?;
    Ok(Some(PlayerId::new(s, g)))
}

/// One column over the ticks: `f` reads the field, `order` its prediction.
fn tick_column(w: &mut Writer, ticks: &[TickRecord], order: u8, f: impl Fn(&TickRecord) -> i64) {
    let mut last = (0_i64, 0_i64);
    put(
        w,
        ticks.iter().map(|t| {
            let v = f(t);
            let step = if order == 2 { last.0 - last.1 } else { 0 };
            let out = (v, last.0, step);
            last = (v, last.0);
            out
        }),
    );
}

/// One column over every man of every tick, predicted from the man in the
/// same place of the list at the tick before (the list is ordered by id:
/// only a substitution moves it, and then the column only costs more).
fn agent_column(w: &mut Writer, ticks: &[TickRecord], order: u8, f: impl Fn(&AgentFrame) -> i64) {
    let mut prev: Vec<i64> = Vec::new();
    let mut prev2: Vec<i64> = Vec::new();
    for t in ticks {
        let now: Vec<i64> = t.agents.iter().map(&f).collect();
        for (i, &v) in now.iter().enumerate() {
            let p1 = prev.get(i).copied().unwrap_or(0);
            let p0 = prev2.get(i).copied().unwrap_or(p1);
            let step = if order == 2 { p1 - p0 } else { 0 };
            w.int(v - p1 - step);
        }
        prev2 = std::mem::replace(&mut prev, now);
    }
}

fn read_tick_column(r: &mut Reader<'_>, n: usize, order: u8) -> Result<Vec<i64>, RecordError> {
    let mut out = Vec::with_capacity(n);
    let mut last = (0_i64, 0_i64);
    for _ in 0..n {
        let step = if order == 2 { last.0 - last.1 } else { 0 };
        let v = get(r, last.0, step)?;
        out.push(v);
        last = (v, last.0);
    }
    Ok(out)
}

fn read_agent_column(
    r: &mut Reader<'_>,
    counts: &[usize],
    order: u8,
) -> Result<Vec<Vec<i64>>, RecordError> {
    let mut out: Vec<Vec<i64>> = Vec::with_capacity(counts.len());
    let empty = Vec::new();
    for (k, &n) in counts.iter().enumerate() {
        let prev = if k > 0 { &out[k - 1] } else { &empty };
        let prev2 = if k > 1 { &out[k - 2] } else { &empty };
        let mut now = Vec::with_capacity(n);
        for i in 0..n {
            let p1 = prev.get(i).copied().unwrap_or(0);
            let p0 = prev2.get(i).copied().unwrap_or(p1);
            let step = if order == 2 { p1 - p0 } else { 0 };
            now.push(get(r, p1, step)?);
        }
        out.push(now);
    }
    Ok(out)
}

fn as_i64<T: TryInto<i64>>(v: T) -> i64 {
    v.try_into().unwrap_or(i64::MAX)
}

fn narrow<T: TryFrom<i64>>(v: i64) -> Result<T, RecordError> {
    T::try_from(v).map_err(|_| RecordError::Damaged)
}

/// The columns of format v1, in their order. A new field is a new column
/// appended here, and read in [`unpack_columns`] only when the record's
/// format has it.
fn pack_columns(ticks: &[TickRecord]) -> Vec<u8> {
    let mut w = Writer::default();
    tick_column(&mut w, ticks, 2, |t| as_i64(t.tick));
    tick_column(&mut w, ticks, 1, |t| half_code(t.half));
    tick_column(&mut w, ticks, 2, |t| i64::from(t.half_seconds));
    for k in 0..3 {
        tick_column(&mut w, ticks, 2, |t| {
            let p = t.ball.position;
            i64::from(<[i32; 3]>::from(p)[k])
        });
    }
    tick_column(&mut w, ticks, 1, |t| state_code(t.ball.state));
    tick_column(&mut w, ticks, 1, |t| id_code(t.carrier).0);
    tick_column(&mut w, ticks, 1, |t| id_code(t.carrier).1);
    tick_column(&mut w, ticks, 1, |t| i64::from(t.score.home));
    tick_column(&mut w, ticks, 1, |t| i64::from(t.score.away));
    tick_column(&mut w, ticks, 1, |t| as_i64(t.events.start));
    tick_column(&mut w, ticks, 1, |t| as_i64(t.events.end));
    tick_column(&mut w, ticks, 1, |t| as_i64(t.agents.len()));
    agent_column(&mut w, ticks, 1, |a| id_code(Some(a.id)).0);
    agent_column(&mut w, ticks, 1, |a| id_code(Some(a.id)).1);
    agent_column(&mut w, ticks, 2, |a| i64::from(a.position.0));
    agent_column(&mut w, ticks, 2, |a| i64::from(a.position.1));
    agent_column(&mut w, ticks, 1, |a| i64::from(a.heading));
    agent_column(&mut w, ticks, 1, |a| i64::from(a.fatigue_milli));
    w.0
}

#[allow(clippy::similar_names)] // the columns are named after their fields
fn unpack_columns(bytes: &[u8], n: usize, _format: u32) -> Result<Vec<TickRecord>, RecordError> {
    let mut r = Reader { bytes, at: 0 };
    let tick = read_tick_column(&mut r, n, 2)?;
    let half = read_tick_column(&mut r, n, 1)?;
    let secs = read_tick_column(&mut r, n, 2)?;
    let bx = read_tick_column(&mut r, n, 2)?;
    let by = read_tick_column(&mut r, n, 2)?;
    let bz = read_tick_column(&mut r, n, 2)?;
    let state = read_tick_column(&mut r, n, 1)?;
    let cslot = read_tick_column(&mut r, n, 1)?;
    let cgen = read_tick_column(&mut r, n, 1)?;
    let home = read_tick_column(&mut r, n, 1)?;
    let away = read_tick_column(&mut r, n, 1)?;
    let ev0 = read_tick_column(&mut r, n, 1)?;
    let ev1 = read_tick_column(&mut r, n, 1)?;
    let counts: Vec<usize> = read_tick_column(&mut r, n, 1)?
        .into_iter()
        .map(narrow)
        .collect::<Result<_, _>>()?;
    let aslot = read_agent_column(&mut r, &counts, 1)?;
    let agen = read_agent_column(&mut r, &counts, 1)?;
    let ax = read_agent_column(&mut r, &counts, 2)?;
    let ay = read_agent_column(&mut r, &counts, 2)?;
    let head = read_agent_column(&mut r, &counts, 1)?;
    let fat = read_agent_column(&mut r, &counts, 1)?;
    // Columns a later format appends would be read here, `if _format >= 2`;
    // an older record gives them their defaults.
    if r.at != bytes.len() {
        return Err(RecordError::Damaged);
    }
    (0..n)
        .map(|k| {
            let agents = (0..counts[k])
                .map(|i| {
                    Ok(AgentFrame {
                        id: id_of(aslot[k][i], agen[k][i])?.ok_or(RecordError::Damaged)?,
                        position: (narrow(ax[k][i])?, narrow(ay[k][i])?),
                        heading: narrow(head[k][i])?,
                        fatigue_milli: narrow(fat[k][i])?,
                    })
                })
                .collect::<Result<Vec<_>, RecordError>>()?;
            Ok(TickRecord {
                tick: narrow(tick[k])?,
                half: half_of(half[k])?,
                half_seconds: narrow(secs[k])?,
                ball: BallFrame {
                    position: (narrow(bx[k])?, narrow(by[k])?, narrow(bz[k])?),
                    state: state_of(state[k])?,
                },
                carrier: id_of(cslot[k], cgen[k])?,
                agents,
                score: Score {
                    home: narrow(home[k])?,
                    away: narrow(away[k])?,
                },
                events: narrow(ev0[k])?..narrow(ev1[k])?,
            })
        })
        .collect()
}

// ---- base64 (the save is JSON) --------------------------------------------

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn to_base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for k in 0..4 {
            if k <= chunk.len() {
                out.push(char::from(ALPHABET[((n >> (18 - 6 * k)) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}

fn from_base64(s: &str) -> Result<Vec<u8>, RecordError> {
    let value = |c: u8| -> Result<u32, RecordError> {
        ALPHABET
            .iter()
            .position(|&a| a == c)
            .and_then(|p| u32::try_from(p).ok())
            .ok_or(RecordError::Damaged)
    };
    let bytes = s.as_bytes();
    if bytes.len() % 4 != 0 {
        return Err(RecordError::Damaged);
    }
    // Padding, if any, is only valid in the final chunk and only as 0, 1 or
    // 2 trailing '=' (V-01): more than two, or padding in a non-final chunk,
    // is malformed and must not reach the slice arithmetic below.
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for (i, chunk) in bytes.chunks(4).enumerate() {
        let is_last = i == bytes.len() / 4 - 1;
        let pad = chunk.iter().rev().take_while(|&&c| c == b'=').count();
        if pad > 2 {
            return Err(RecordError::Damaged);
        }
        if pad > 0 && !is_last {
            return Err(RecordError::Damaged);
        }
        // A non-final chunk with any '=' is also malformed.
        if pad > 0 && chunk[..4 - pad].contains(&b'=') {
            return Err(RecordError::Damaged);
        }
        let mut n = 0_u32;
        for &c in &chunk[..4 - pad] {
            n = (n << 6) | value(c)?;
        }
        n <<= 6 * u32::try_from(pad).map_err(|_| RecordError::Damaged)?;
        let three = n.to_be_bytes();
        let three = [three[1], three[2], three[3]];
        out.extend_from_slice(&three[..3 - pad]);
    }
    Ok(out)
}

impl PackedRecord {
    /// Packs the tick record of a match played now.
    #[must_use]
    pub fn pack(header: RecordHeader, ticks: &[TickRecord]) -> Self {
        let columns = pack_columns(ticks);
        let deflated = miniz_oxide::deflate::compress_to_vec(&columns, 9);
        Self {
            header,
            ticks: u32::try_from(ticks.len()).unwrap_or(u32::MAX),
            data: to_base64(&deflated),
        }
    }

    /// Unpacks the tick record, exactly as it was packed.
    ///
    /// # Errors
    ///
    /// [`RecordError::FormatTooNew`] for a record of a newer format,
    /// [`RecordError::Damaged`] if the data does not read back.
    pub fn unpack(&self) -> Result<Vec<TickRecord>, RecordError> {
        // The most ticks a recorded match can plausibly have (V-02): about
        // 120 minutes at 10 ticks/s, plus a generous margin for extra time.
        const MAX_TICKS: u32 = 200_000;
        let format = self.header.format_version;
        if format > RECORD_FORMAT_VERSION {
            return Err(RecordError::FormatTooNew {
                record: format,
                current: RECORD_FORMAT_VERSION,
            });
        }
        if self.ticks > MAX_TICKS {
            return Err(RecordError::Damaged);
        }
        let deflated = from_base64(&self.data)?;
        let columns = miniz_oxide::inflate::decompress_to_vec_with_limit(&deflated, 1 << 30)
            .map_err(|_| RecordError::Damaged)?;
        unpack_columns(&columns, self.ticks as usize, format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn played(seed: u64) -> (ds_model::Match, Vec<TickRecord>) {
        let m = ds_fixtures::match_fixture(seed).expect("fixture");
        let r = crate::simulate_from_seed_recorded(&m, seed, crate::MatchConfig::default()).1;
        (m, r)
    }

    /// A whole match packs and unpacks to the same record, to the last
    /// tick, and it is small.
    #[test]
    fn a_match_comes_back_exactly() {
        let (m, r) = played(42);
        let header = RecordHeader::current(10, (m.pitch.length_m, m.pitch.width_m));
        let packed = PackedRecord::pack(header, &r);
        eprintln!("packed: {} bytes", packed.data.len());
        // About 2.3 MB at SIM 147, with the bodies in small steps (ADR-069).
        assert!(packed.data.len() < 3_000_000, "{} bytes", packed.data.len());
        assert_eq!(packed.unpack().expect("unpack"), r);
    }

    /// A record from a newer format is refused, not misread.
    #[test]
    fn a_newer_format_is_refused() {
        let (m, r) = played(42);
        let mut header = RecordHeader::current(10, (m.pitch.length_m, m.pitch.width_m));
        header.format_version = RECORD_FORMAT_VERSION + 1;
        let packed = PackedRecord::pack(header, &r[..20]);
        assert!(matches!(
            packed.unpack(),
            Err(RecordError::FormatTooNew { .. })
        ));
    }

    /// Damaged data is an error, never a panic or a wrong match.
    #[test]
    fn damaged_data_is_an_error() {
        let (m, r) = played(42);
        let header = RecordHeader::current(10, (m.pitch.length_m, m.pitch.width_m));
        let mut packed = PackedRecord::pack(header, &r[..50]);
        packed.ticks += 1;
        assert_eq!(packed.unpack(), Err(RecordError::Damaged));
        packed.ticks -= 1;
        packed.data.truncate(packed.data.len() - 8);
        assert_eq!(packed.unpack(), Err(RecordError::Damaged));
    }

    /// The record of format v1 written on 16 November 2026 still reads:
    /// the first ticks of seed 42, frozen. If this fails, the format
    /// changed without its version (ADR-065).
    #[test]
    fn the_v1_record_still_reads() {
        let frozen: PackedRecord =
            serde_json::from_str(include_str!("../tests/fixtures/record-v1.json"))
                .expect("frozen v1 record");
        assert_eq!(frozen.header.format_version, 1);
        let ticks = frozen.unpack().expect("v1 reads");
        assert_eq!(ticks.len(), 30);
        assert_eq!(ticks[0].tick, 0);
        assert_eq!(ticks[0].agents.len(), 22);
    }

    #[test]
    fn base64_round_trips() {
        for n in 0..10 {
            let v: Vec<u8> = (0..n)
                .map(|i: u8| i.wrapping_mul(37).wrapping_add(11))
                .collect();
            assert_eq!(from_base64(&to_base64(&v)).expect("b64"), v);
        }
    }
}
