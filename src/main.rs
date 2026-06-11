//! kobold-invariant-rank CLI: rank court candidates from a gap board + an atlas directory.
//!   kobold-invariant-rank <gap-board.json> <atlas-dir>
use kobold_archaeology::GapBoard;
use kobold_atlas::AtlasLibrary;
use kobold_invariant_rank::{from_gap_and_atlas, rank};
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: kobold-invariant-rank <gap-board.json> <atlas-dir>");
        exit(2);
    }
    let gap = match GapBoard::load(&args[1]) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("gap-board load error: {e}");
            exit(2);
        }
    };
    let atlas = match AtlasLibrary::load_dir(&args[2]) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("atlas load error: {e}");
            exit(2);
        }
    };
    let ranked = rank(&from_gap_and_atlas(&gap, &atlas));
    println!("ranked court candidates (occurrence x migration-risk x oracle-sharpness):");
    for r in ranked.iter().take(20) {
        println!(
            "  {:>7.2}  {:<28} {:<9} occ={:<7} {}",
            r.score, r.surface, r.status, r.occurrences, r.recommendation
        );
    }
}
