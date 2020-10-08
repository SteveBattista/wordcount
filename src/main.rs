use std::collections::BTreeMap;
use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let word_re = Regex::new(r"[a-z\-0-9']+").unwrap();

    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    let stdin = io::stdin();
    stdin.lock().lines().for_each(|line| {
        let line = line.unwrap().to_lowercase();
        let matches = word_re.find_iter(&line);

        let words = matches.map(|mat| mat.as_str());

        words.for_each(|word| {
            *counts.entry(word.into()).or_insert(0) += 1;
        });
    });
    let mut total = 0;
    let mut unique = 0;
    counts.iter().for_each(|(key, value)| {
        total += value;
        unique += 1;
        println!(" {} shows up {} times.", key, value);
    });
    println!(
        "Number of unique words was {} out of a total of {} words.",
        unique, total
    );
}
