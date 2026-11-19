//! The goalkeeper's depth off his line (plan L0-CLOSE 10d, ADR-078).
//!
//! **One responsibility:** how far the keeper leaves his line, from the
//! geometry of the ball and his own reading of the play. Pure: the caller
//! gives the numbers. No die (ADR-035).

/// How far off his line the keeper stands, cm (10d).
///
/// `advance_cm` is the depth a keeper who reads the play fully takes at
/// `t = 1` (`keeper.json`); `t` is how close the ball is to his goal; `read`
/// is his own game reading. A keeper who does not read the play stays on
/// his line, ready to react to the shot; one who reads it comes out to
/// narrow the angle and claim the space behind the defence. The choice is
/// his, not a rule imposed on every keeper alike.
#[must_use]
pub fn line_depth_cm(advance_cm: f64, t: f64, read: f64) -> f64 {
    advance_cm.max(0.0) * t.clamp(0.0, 1.0) * read.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::line_depth_cm;

    /// Twin test (10d): two keepers, the same ball, the same depth budget;
    /// only the reading differs. The one who reads the play comes out, the
    /// other stays ready on his line.
    #[test]
    fn the_reader_comes_off_his_line() {
        let reader = line_depth_cm(400.0, 1.0, 0.9);
        let waiter = line_depth_cm(400.0, 1.0, 0.1);
        assert!(reader > waiter + 200.0, "reader {reader}, waiter {waiter}");
        assert!(waiter < 50.0, "the poor reader waits on his line: {waiter}");
    }

    #[test]
    fn a_ball_far_away_keeps_him_on_his_line() {
        assert_eq!(line_depth_cm(400.0, 0.0, 0.9), 0.0);
    }
}
