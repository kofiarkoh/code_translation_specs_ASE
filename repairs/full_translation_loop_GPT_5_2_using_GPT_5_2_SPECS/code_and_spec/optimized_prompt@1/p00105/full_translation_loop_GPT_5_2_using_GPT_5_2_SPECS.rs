use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

fn main() {
    // Consume stdin as whitespace-delimited tokens: (word, page) pairs.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Keep words sorted and pages sorted/deduplicated per spec.
    let mut index: BTreeMap<String, BTreeSet<u16>> = BTreeMap::new();

    while let (Some(word), Some(page_s)) = (it.next(), it.next()) {
        // Preconditions guarantee valid integer in [0, 1000].
        // If violated, behavior is unspecified; we simply stop.
        let page: u16 = match page_s.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        index.entry(word.to_string()).or_default().insert(page);
    }

    // Match the C program's behavior: if there are no entries, print nothing at all.
    if index.is_empty() {
        return;
    }

    // Emit output with exact spacing rules: single spaces, no trailing space.
    let mut out = String::new();
    for (word, pages) in index {
        out.push_str(&word);
        out.push('\n');

        let mut iter = pages.into_iter();
        if let Some(first) = iter.next() {
            out.push_str(&first.to_string());
            for p in iter {
                out.push(' ');
                out.push_str(&p.to_string());
            }
        }
        out.push('\n');
    }

    print!("{out}");
}
