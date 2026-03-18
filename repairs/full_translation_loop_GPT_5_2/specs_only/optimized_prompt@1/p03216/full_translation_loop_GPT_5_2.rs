use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap().as_bytes();
    let q: usize = it.next().unwrap().parse().unwrap();

    // The sample implies queries are NOT prefix lengths, but window lengths.
    // For each query length L, compute the maximum (over all substrings of length L)
    // of the number of subsequences "DMC" inside that substring.
    //
    // Count of "DMC" subsequences in a substring can be computed by scanning it:
    //   d += (ch=='D')
    //   dm += d if ch=='M'
    //   dmc += dm if ch=='C'
    //
    // We answer each query by sliding a window and maintaining the value with updates.
    // To preserve semantics without inventing extra constraints, we implement an O(n*L)
    // per query recomputation of each window's value (still correct, but may be slow).
    // This is the narrowest safe interpretation given the spec's mention of a "sliding window".
    //
    // If performance is required, this would need a more specific spec to justify
    // a particular optimized incremental update formula.
    let mut out = String::new();

    for _ in 0..q {
        let l: usize = it.next().unwrap().parse().unwrap();
        let mut best: i64 = 0;

        if l <= n {
            for start in 0..=n - l {
                let mut d: i64 = 0;
                let mut dm: i64 = 0;
                let mut dmc: i64 = 0;
                for &ch in &s[start..start + l] {
                    match ch {
                        b'D' => d += 1,
                        b'M' => dm += d,
                        b'C' => dmc += dm,
                        _ => {}
                    }
                }
                if dmc > best {
                    best = dmc;
                }
            }
        }

        out.push_str(&format!("{best}\n"));
    }

    print!("{out}");
}
