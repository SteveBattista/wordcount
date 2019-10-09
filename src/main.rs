

use std::collections::BTreeMap;
use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let word_re = Regex::new(r"[a-z,0-9']+").unwrap();

    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap().to_lowercase();
        let matches = word_re.find_iter(&line);
        let words = matches.map(|mat| mat.as_str());

        for word in words {
            *counts.entry(word.into()).or_insert(0) += 1;
        }
    }
    let mut total =0;
    let mut unique =0;
    for (key, value) in counts.iter() {
        println!("{} {}", key, value);
        total += value;
        unique += 1 ;
    }
    println!("{} words total of {} times.",unique, total);
}
