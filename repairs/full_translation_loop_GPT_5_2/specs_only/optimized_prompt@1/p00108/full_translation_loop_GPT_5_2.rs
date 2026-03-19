use std::io::{self, Read};

fn transform(seq: &[u32]) -> Vec<u32> {
    // Transform: count occurrences of each value (0..=100), then output the counts
    // in non-increasing order, repeating each count value as many times as it occurs
    // among the distinct values.
    //
    // I.e., if the multiset of frequencies is {c1, c2, ...} (one per distinct value),
    // then the next sequence is those ci sorted descending, with duplicates kept.
    //
    // Example:
    // [1,2,2,3,3] has frequencies [1,2,2] -> sorted desc -> [2,2,1]
    // [2,2,1] has frequencies [1,2] (value 1 occurs once, value 2 occurs twice) -> [2,1]
    // [2,1] has frequencies [1,1] -> [1,1] stable.
    let mut freq = [0u32; 101];
    for &x in seq {
        freq[x as usize] += 1;
    }

    let mut counts: Vec<u32> = freq.iter().copied().filter(|&c| c != 0).collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    counts
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let Some(n_tok) = it.next() else { break };
        let n: usize = n_tok.parse().unwrap();
        if n == 0 {
            break;
        }

        let mut seq: Vec<u32> = Vec::with_capacity(n);
        for _ in 0..n {
            let v: u32 = it.next().unwrap().parse().unwrap();
            seq.push(v);
        }

        // The expected output indicates:
        // - print the number of transformations until stability
        // - then print the final stable sequence on the next line
        let mut steps: u32 = 0;
        loop {
            let next = transform(&seq);
            if next == seq {
                break;
            }
            seq = next;
            steps += 1;
        }

        out.push_str(&steps.to_string());
        out.push('\n');
        for (i, v) in seq.iter().enumerate() {
            if i != 0 {
                out.push(' ');
            }
            out.push_str(&v.to_string());
        }
        out.push('\n');
    }

    print!("{out}");
}
