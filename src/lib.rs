#![forbid(unsafe_code)]
//! # kobold-invariant-rank
//!
//! The decision engine. It ranks COBOL invariants by **real-ecosystem occurrence** x **oracle-sharpness**
//! x **migration-risk**, so the limited budget of "build the next court" goes to the surfaces that matter
//! most: heavily used in real code, sharply discriminated by the oracle, and not yet covered.
//!
//! Bridges [`kobold_archaeology`] (occurrence terrain) and [`kobold_atlas`] (sharpness signal). Part of the
//! KOBOLD ecosystem (Apache-2.0). Dependency rule: kobold-* MAY depend on gnucobol-rs; never the reverse.

use kobold_archaeology::GapBoard;
use kobold_atlas::AtlasLibrary;
use serde::{Deserialize, Serialize};

/// One scored input: a surface with its occurrence terrain, coverage status, and oracle-sharpness signal.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RankInput {
    pub surface: String,
    pub occurrences: i64,
    /// `sealed` | `observed` | `refused` | `missing`.
    pub status: String,
    pub court: String,
    /// Oracle-sharpness multiplier (1.0 = ordinary; >1 = the oracle draws a sharp, surprising line here).
    pub sharpness: f64,
}

/// A ranked court candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranked {
    pub surface: String,
    pub score: f64,
    pub occurrences: i64,
    pub status: String,
    pub court: String,
    pub recommendation: String,
}

/// Migration-risk weight by coverage status: exercised-but-uncovered is the sharpest priority.
fn risk_weight(status: &str) -> f64 {
    match status {
        "missing" => 3.0,  // real code uses it, no court yet -> highest priority
        "observed" => 1.5, // mapped but not byte-sealed
        "refused" => 1.0,  // deliberately out of scope
        "sealed" => 0.4,   // already covered -> low remaining value
        _ => 1.0,
    }
}

fn occurrence_weight(occ: i64) -> f64 {
    (1.0 + occ.max(0) as f64).ln()
}

fn recommend(status: &str, score: f64) -> String {
    match status {
        "missing" if score >= 8.0 => "BUILD COURT NOW (hot, uncovered)".into(),
        "missing" => "build court (uncovered)".into(),
        "observed" => "consider byte-sealing".into(),
        "sealed" => "covered".into(),
        "refused" => "out of scope (deliberate)".into(),
        _ => "review".into(),
    }
}

/// Rank inputs by `occurrence_weight x risk_weight x sharpness`, hottest first.
pub fn rank(inputs: &[RankInput]) -> Vec<Ranked> {
    let mut out: Vec<Ranked> = inputs
        .iter()
        .map(|i| {
            let score = occurrence_weight(i.occurrences) * risk_weight(&i.status) * i.sharpness.max(0.0);
            Ranked {
                surface: i.surface.clone(),
                score: (score * 100.0).round() / 100.0,
                occurrences: i.occurrences,
                status: i.status.clone(),
                court: i.court.clone(),
                recommendation: recommend(&i.status, score),
            }
        })
        .collect();
    out.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    out
}

/// Build rank inputs from a gap board (occurrence + status) and an atlas library (sharpness signal):
/// a surface whose court carries a locked port-vs-oracle divergence, or a rich refusal ledger, is sharper.
pub fn from_gap_and_atlas(gap: &GapBoard, atlas: &AtlasLibrary) -> Vec<RankInput> {
    // sharpness per court id: 2.0 if it has any locked divergence, else 1.0 + 0.1 per refusal (capped).
    gap.surfaces
        .iter()
        .map(|s| {
            let sharp = atlas
                .atlases
                .iter()
                .find(|a| !s.court.is_empty() && a.court == s.court || a.court == s.surface)
                .map(|a| {
                    if !a.rust_divergence_set.is_empty() {
                        2.0
                    } else {
                        (1.0 + 0.1 * a.negative_capabilities.len() as f64).min(2.0)
                    }
                })
                .unwrap_or(1.0);
            RankInput {
                surface: s.surface.clone(),
                occurrences: s.occurrences,
                status: s.status.clone(),
                court: s.court.clone(),
                sharpness: sharp,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inp(surface: &str, occ: i64, status: &str, sharp: f64) -> RankInput {
        RankInput {
            surface: surface.into(),
            occurrences: occ,
            status: status.into(),
            court: String::new(),
            sharpness: sharp,
        }
    }

    #[test]
    fn missing_hot_outranks_sealed_hot() {
        let r = rank(&[
            inp("MOVE", 9000, "sealed", 1.0),
            inp("CALL", 4200, "missing", 1.0),
        ]);
        assert_eq!(r[0].surface, "CALL", "uncovered hot surface must rank first");
        assert!(r[0].recommendation.contains("BUILD COURT"));
        assert_eq!(r[1].recommendation, "covered");
    }

    #[test]
    fn sharpness_breaks_ties() {
        let r = rank(&[
            inp("A", 1000, "missing", 1.0),
            inp("B", 1000, "missing", 2.0),
        ]);
        assert_eq!(r[0].surface, "B");
        assert!(r[0].score > r[1].score);
    }

    #[test]
    fn zero_occurrence_is_lowest() {
        let r = rank(&[inp("X", 0, "missing", 5.0), inp("Y", 5, "missing", 1.0)]);
        assert_eq!(r[1].surface, "X");
    }
}
