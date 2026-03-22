use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Item {
    a: i64,
    b: i64,
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say inputs are non-negative decimal integers (no leading sign).
    // We parse into u64 first, then cast to i64 (assumed to fit by preconditions).
    let n: usize = match it.next() {
        Some(s) => s.parse::<u64>().unwrap() as usize,
        None => return, // unspecified; keep minimal behavior
    };

    let mut items: Vec<Item> = Vec::with_capacity(n);
    for _ in 0..n {
        let a_u = it.next().unwrap().parse::<u64>().unwrap();
        let b_u = it.next().unwrap().parse::<u64>().unwrap();
        items.push(Item {
            a: a_u as i64,
            b: b_u as i64,
        });
    }

    // Sort by first value ascending, then second value ascending.
    items.sort_by(|x, y| match x.a.cmp(&y.a) {
        Ordering::Equal => x.b.cmp(&y.b),
        other => other,
    });

    // We need:
    // max_{l<=r} (sum_{i=l..r} B_i - (A_r - A_l))
    // = max_{r} ( (prefixB[r] - A_r) - min_{l<=r}(prefixB[l-1] - A_l) )
    // where prefixB[k] = sum_{i=1..k} B_i, prefixB[0]=0.
    //
    // For fixed r:
    // value(l,r) = (prefixB[r] - A_r) - (prefixB[l-1] - A_l)
    // So maintain min of (prefixB[l-1] - A_l) over l up to r.
    //
    // All arithmetic is i64; preconditions state result fits i64.
    let mut prefix: i64 = 0;
    let mut best: i64 = i64::MIN;

    // Initialize min_term using l=1: prefixB[0] - A_1 = -A_1
    // If n==0, behavior is unspecified; we output 0 as a narrow, harmless choice.
    if n == 0 {
        println!("0");
        return;
    }

    let mut min_term: i64 = -items[0].a;

    for (idx, item) in items.iter().enumerate() {
        // r = idx+1
        prefix = prefix + item.b;

        // candidate using current min_term (which corresponds to some l<=r)
        let candidate = (prefix - item.a) - min_term;
        if candidate > best {
            best = candidate;
        }

        // Update min_term for future r using l = (r+1) possibility:
        // term for l = (idx+2) would be prefixB[r] - A_{r+1}, but we don't know A_{r+1} yet.
        // Instead, for next iteration (r' = r+1), we need to consider l = r'+1? no.
        // We need min over l<=r' of (prefixB[l-1] - A_l).
        // When moving from r to r+1, we can add the new l = r+1 term:
        // prefixB[r] - A_{r+1}. We'll compute it at the start of next iteration by using
        // current prefix and next item's A. To keep it simple, update min_term here
        // for l = r+1 using current prefix and next A if exists.
        if idx + 1 < n {
            let next_a = items[idx + 1].a;
            let new_term = prefix - next_a;
            if new_term < min_term {
                min_term = new_term;
            }
        }
    }

    println!("{}", best);
}
