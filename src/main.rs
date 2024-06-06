extern crate strsim;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use strsim::jaro;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename = "english.txt";
    let query = "WONDERFU";
    let threshold = 0.87;

    if let Ok(lines) = read_lines(filename) {
        // Store the start time
        let start = std::time::Instant::now();
        for line in lines {
            if let Ok(content) = line {
                let similarity = jaro(&query, &content);
                if similarity >= threshold {
                    println!("Match: {} (similarity: {})", content, similarity);
                }
            }
        }
        // Calculate the elapsed time
        let elapsed = start.elapsed();
        println!("Elapsed time: {:?}", elapsed);
        println!("Done!");
    } else {
        println!("Could not read file: {}", filename);
    }
}
