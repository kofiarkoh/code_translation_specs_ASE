use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

fn main() {
    // Spec-driven approach:
    // - Aggregate by word
    // - Deduplicate page numbers per word
    // - Output words in lexicographic order, pages in ascending numeric order
    //
    // Note: The C code increments `tm` even for duplicate (word,page) pairs, which can
    // cause incorrect output (it may fail to print the final newline or stop early).
    // The specification requires duplicates not to create duplicate page numbers, so
    // we implement set semantics here.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut index: BTreeMap<String, BTreeSet<i32>> = BTreeMap::new();

    loop {
        let Some(word) = it.next() else { break };
        let Some(page_s) = it.next() else { break }; // incomplete pair: unspecified; stop
        let page: i32 = match page_s.parse() {
            Ok(v) => v,
            Err(_) => break, // unspecified; stop
        };

        index.entry(word.to_string()).or_default().insert(page);
    }

    let mut out = String::new();
    for (word, pages) in index {
        out.push_str(&word);
        out.push('\n');

        let mut first = true;
        for p in pages {
            if !first {
                out.push(' ');
            }
            first = false;
            out.push_str(&p.to_string());
        }
        out.push('\n');
    }

    print!("{out}");
}
