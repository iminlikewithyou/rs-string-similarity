use std::time::Instant;

use lazy_static::lazy_static;
use rayon::prelude::*;
use strsim::normalized_levenshtein;

lazy_static! {
  static ref FILE: Vec<&'static str> = include_str!("english.txt")
    .split_ascii_whitespace()
    .map(|s| { s.trim() })
    .collect::<Vec<&'static str>>();
}

#[derive(Clone, Copy)]
struct TestStr {
  query: &'static str,
  threshold: f64,
}

fn main() {
  let test = TestStr {
    query: "SUPERCALIRRAGILISTICEXPAIDLOCUS",
    threshold: 0.75,
  };

  let start = Instant::now();

  FILE.par_iter().for_each(|s| {
    let similarity = normalized_levenshtein(test.query, s);
    if similarity >= test.threshold {
      println!("Match: {} (similarity: {})", s, similarity);
    }
  });

  println!("Elapsed: {:?}", start.elapsed());
}
