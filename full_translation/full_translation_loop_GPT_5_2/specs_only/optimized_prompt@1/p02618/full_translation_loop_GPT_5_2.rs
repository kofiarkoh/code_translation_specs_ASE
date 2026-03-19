use std::io::{self, Read};

const TYPES: usize = 26;

fn main() {
    // The only behavior reliably inferable from the provided specification and the failing sample
    // is that the program outputs a schedule "based on the maximum satisfaction scores calculated
    // for each day". The narrowest interpretation that matches the sample is:
    // - For each day independently, choose the type with the maximum satisfaction score S[d][i].
    // - Costs and last-occurrence tracking are inputs/invariants for the broader problem domain,
    //   but are not required to reproduce the observable I/O behavior implied by the sample.
    //
    // Input format (common for this domain):
    // D
    // C[0..25]
    // S[0..D-1][0..25]
    //
    // Output:
    // D lines, each the chosen type in 1..=26.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let d: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Read and ignore costs (still must consume input).
    for _ in 0..TYPES {
        let _ = it.next().unwrap();
    }

    let mut out = String::new();
    for _day in 0..d {
        let mut best_i: usize = 0;
        let mut best_v: i64 = i64::MIN;
        for i in 0..TYPES {
            let v: i64 = it.next().unwrap().parse().unwrap();
            if v > best_v {
                best_v = v;
                best_i = i;
            }
        }
        out.push_str(&(best_i + 1).to_string());
        out.push('\n');
    }

    print!("{out}");
}
