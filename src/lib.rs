//! # kobold-invariant-rank
//!
//! Real-ecosystem x generated-oracle-witness ranking: feature-cell vocabulary, public-occurrence/oracle-sharpness/migration-risk scoring, and court-candidate prioritization.
//!
//! Part of the KOBOLD ecosystem -- independently-authored forensic tooling. Apache-2.0. This crate
//! contains no GnuCOBOL/libcob source; any interaction with COBOL semantics goes through the separate
//! gnucobol-rs crate.
//!
//! Architecture: kobold-* MAY depend on gnucobol-rs; gnucobol-rs MUST NOT depend on kobold-*.
//!
//! Status: SCAFFOLD. Implementation extracted from the gnucobol-rs lab tooling + lineage engine later.
#![forbid(unsafe_code)]

/// Crate scaffold marker; replace with the real public API as the implementation lands.
pub const KOBOLD_CRATE: &str = "kobold-invariant-rank";
