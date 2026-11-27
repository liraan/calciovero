//! The reference bundle: one truth declaration per metric, from any number
//! of real-match sources (ADR-116).
//!
//! A bundle is data, not a provider in the code. Each source says where its
//! values come from, whether it observes real matches, its granularity and
//! whether its rows are attributable to a player who chose what to do. The
//! `truth` map says which source supplies each metric. An aggregate of a
//! whole team may be listed as a cross-check, but the builder refuses it as
//! truth: a team total is not a player reading the match and choosing COME.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

use crate::Metric;

/// The data of a bundle: sources plus the source chosen for each metric.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    label: String,
    sources: BTreeMap<String, Source>,
    truth: BTreeMap<String, String>,
}

/// The declaration of one real-match source.
#[derive(Debug, Deserialize)]
pub struct Source {
    file: String,
    partita_vera: bool,
    granularity: String,
    actor_attributable: bool,
    licence: String,
}

/// Reads the bundle manifest.
///
/// # Errors
/// When the file cannot be read or is not a valid manifest.
pub fn read_manifest(path: &str) -> Result<Manifest, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))
}

/// Loads every source JSON named by the manifest, relative to its directory.
///
/// # Errors
/// When a source file is missing or is not valid JSON.
pub fn load_sources(path: &str, manifest: &Manifest) -> Result<BTreeMap<String, Value>, String> {
    let dir = Path::new(path).parent().unwrap_or_else(|| Path::new("."));
    let mut loaded = BTreeMap::new();
    for (id, source) in &manifest.sources {
        let file = dir.join(&source.file);
        let name = file.to_string_lossy();
        let text = std::fs::read_to_string(&file).map_err(|e| format!("{name}: {e}"))?;
        let value: Value = serde_json::from_str(&text).map_err(|e| format!("{name}: {e}"))?;
        loaded.insert(id.clone(), value);
    }
    Ok(loaded)
}

/// Flattens the bundle into the reference shape that `compare` already reads.
///
/// # Errors
/// When a metric has no truth declaration, its source fails the partita-vera
/// or LUNA filter, or the source file does not carry that metric.
pub fn build(manifest: &Manifest, sources: &BTreeMap<String, Value>) -> Result<Value, String> {
    let mut declared = BTreeMap::new();
    for (id, source) in &manifest.sources {
        let label = sources
            .get(id)
            .and_then(|v| v["label"].as_str())
            .unwrap_or(id)
            .to_owned();
        declared.insert(
            id.clone(),
            serde_json::json!({
                "label": label,
                "file": source.file,
                "partita_vera": source.partita_vera,
                "granularity": source.granularity,
                "actor_attributable": source.actor_attributable,
                "licence": source.licence,
            }),
        );
    }

    let mut metrics = serde_json::Map::new();
    let mut matches: Option<u64> = None;
    let mut per: Option<String> = None;
    for m in Metric::ALL {
        let id = manifest
            .truth
            .get(m.key())
            .ok_or_else(|| format!("truth is missing for {}", m.key()))?;
        let source = manifest
            .sources
            .get(id)
            .ok_or_else(|| format!("{}: unknown source {id}", m.key()))?;
        if !source.partita_vera {
            return Err(format!("{id}: not a real-match source"));
        }
        if source.granularity == "aggregate" {
            return Err(format!(
                "{id}: a team aggregate cannot be truth for {}",
                m.key()
            ));
        }
        if source.granularity != "event" && source.granularity != "frame" {
            return Err(format!("{id}: unknown granularity {}", source.granularity));
        }
        if !source.actor_attributable {
            return Err(format!("{id}: rows are not attributable to a player"));
        }
        if source.licence.trim().is_empty() {
            return Err(format!("{id}: no licence declared"));
        }
        let doc = sources
            .get(id)
            .ok_or_else(|| format!("{id}: source values not loaded"))?;
        let n = doc["matches"]
            .as_u64()
            .ok_or_else(|| format!("{id}: no match count"))?;
        match matches {
            Some(previous) if previous != n => {
                return Err(format!("{id}: {n} matches, others have {previous}"));
            }
            Some(_) => {}
            None => matches = Some(n),
        }
        if per.is_none() {
            per = Some(
                doc["per"]
                    .as_str()
                    .unwrap_or("match, both teams together")
                    .to_owned(),
            );
        }
        let row = &doc["metrics"][m.key()];
        let mean = row["mean"]
            .as_f64()
            .ok_or_else(|| format!("{id}: {} has no mean", m.key()))?;
        let sd = row["sd"]
            .as_f64()
            .ok_or_else(|| format!("{id}: {} has no sd", m.key()))?;
        let mut out = serde_json::json!({ "mean": mean, "sd": sd, "source": id });
        out["comparable"] = Value::from(
            row["comparable"]
                .as_bool()
                .unwrap_or_else(|| m.comparable()),
        );
        let note = row["note"]
            .as_str()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| m.note());
        if !note.is_empty() {
            out["note"] = Value::from(note);
        }
        metrics.insert(m.key().to_owned(), out);
    }
    Ok(serde_json::json!({
        "label": manifest.label,
        "per": per.unwrap_or_else(|| "match, both teams together".to_owned()),
        "matches": matches.unwrap_or(0),
        "source": { "bundle": manifest.label, "sources": declared },
        "metrics": metrics,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(truth_source: &str) -> Manifest {
        let truth: BTreeMap<String, String> = Metric::ALL
            .iter()
            .map(|m| (m.key().to_owned(), truth_source.to_owned()))
            .collect();
        serde_json::from_value(serde_json::json!({
            "label": "test bundle",
            "sources": {
                "real": {
                    "file": "real.json",
                    "partita_vera": true,
                    "granularity": "event",
                    "actor_attributable": true,
                    "licence": "CC BY 4.0"
                },
                "team-totals": {
                    "file": "totals.json",
                    "partita_vera": true,
                    "granularity": "aggregate",
                    "actor_attributable": false,
                    "licence": "terms apply"
                }
            },
            "truth": truth,
        }))
        .unwrap()
    }

    fn sources() -> BTreeMap<String, Value> {
        let metrics: serde_json::Map<String, Value> = Metric::ALL
            .iter()
            .map(|m| {
                (
                    m.key().to_owned(),
                    serde_json::json!({ "mean": 1.0, "sd": 0.5 }),
                )
            })
            .collect();
        BTreeMap::from([
            (
                "real".to_owned(),
                serde_json::json!({
                    "label": "real matches", "matches": 10,
                    "per": "match, both teams together", "metrics": metrics
                }),
            ),
            (
                "team-totals".to_owned(),
                serde_json::json!({ "label": "team totals", "matches": 10, "metrics": {} }),
            ),
        ])
    }

    #[test]
    fn a_bundle_takes_its_truth_from_the_declared_source() {
        let out = build(&manifest("real"), &sources()).unwrap();
        assert_eq!(out["metrics"][Metric::Goals.key()]["source"], "real");
        assert_eq!(out["metrics"][Metric::Goals.key()]["mean"], 1.0);
        assert_eq!(out["metrics"][Metric::Goals.key()]["comparable"], true);
        assert_eq!(out["source"]["sources"]["real"]["licence"], "CC BY 4.0");
    }

    #[test]
    fn every_metric_needs_a_truth_declaration() {
        let mut m = manifest("real");
        m.truth.remove(Metric::Crosses.key());
        let error = build(&m, &sources()).unwrap_err();
        assert!(error.contains(Metric::Crosses.key()));
    }

    #[test]
    fn a_team_aggregate_is_a_cross_check_not_truth() {
        let m = manifest("team-totals");
        let error = build(&m, &sources()).unwrap_err();
        assert!(error.contains("aggregate"));
    }

    #[test]
    fn the_bundle_keeps_the_historical_numbers() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../L0-match/reference");
        let path = format!("{dir}/bundle.json");
        let manifest = read_manifest(&path).unwrap();
        let sources = load_sources(&path, &manifest).unwrap();
        let built = build(&manifest, &sources).unwrap();
        let text = std::fs::read_to_string(format!("{dir}/serie-a-2017-18.json")).unwrap();
        let historical: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(built["matches"], historical["matches"]);
        for m in Metric::ALL {
            assert_eq!(
                built["metrics"][m.key()]["mean"],
                historical["metrics"][m.key()]["mean"],
                "{} mean",
                m.key()
            );
            assert_eq!(
                built["metrics"][m.key()]["sd"],
                historical["metrics"][m.key()]["sd"],
                "{} sd",
                m.key()
            );
        }
    }
}
