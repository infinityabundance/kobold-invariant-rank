# kobold-invariant-rank

Real-ecosystem x generated-oracle-witness ranking: feature-cell vocabulary, public-occurrence/oracle-sharpness/migration-risk scoring, and court-candidate prioritization.

**Part of KOBOLD** -- a forensic archaeology and evidence system for legacy COBOL estates: it maps real COBOL
codebases, generated oracle witnesses, compiler-profile behavior, and migration risk into court-backed
receipts. Independently-authored tooling; contains no GnuCOBOL source.

## What it does (v0.1)
The bridge between real-code terrain and oracle sharpness:
- `rank(&[RankInput]) -> Vec<Ranked>` scores each surface by `occurrence_weight x migration_risk x sharpness`
  (exercised-but-uncovered ranks highest; sealed ranks lowest), with a build recommendation.
- `from_gap_and_atlas(&GapBoard, &AtlasLibrary)` wires occurrence/status from kobold-archaeology and the
  sharpness signal (locked divergence / refusal richness) from kobold-atlas.
- CLI: `kobold-invariant-rank <gap-board.json> <atlas-dir>`.

```
cargo run -- path/to/public-gap-board.json path/to/reports
```

Composes `kobold-archaeology` + `kobold-atlas` (path deps for now; switch to version deps before publish).
Roadmap: feature-cell vocabulary, generated-witness yield scoring, efficiency metrics.

## Architecture
- gnucobol-rs (separate crate) = the oracle-proven semantic primitive layer.
- kobold-* = the forensic-intelligence layer.
- kobold-* MAY depend on gnucobol-rs; gnucobol-rs MUST NOT depend on kobold-*.

## License
Apache-2.0 (see LICENSE).
