use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions are assumed satisfied; behavior is undefined otherwise.
    let n: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap().as_bytes();
    let q: usize = it.next().unwrap().parse().unwrap();

    // Narrowest reasonable interpretation of the "cumulative value":
    // For a given prefix length L, compute the number of subsequences "DMC"
    // within s[0..L), i.e., count of triples i<j<k with s[i]='D', s[j]='M', s[k]='C'.
    //
    // This matches the described need to maintain counts of 'D' and 'M' while processing.
    // We answer multiple queries efficiently by precomputing prefix answers for all L.
    let mut prefix_ans: Vec<i64> = vec![0; n + 1];
    let mut d_count: i64 = 0;
    let mut dm_count: i64 = 0;
    let mut dmc_count: i64 = 0;

    for i in 0..n {
        match s[i] {
            b'D' => {
                d_count += 1;
            }
            b'M' => {
                // Each 'M' pairs with all previous 'D' to form "DM".
                dm_count += d_count;
            }
            b'C' => {
                // Each 'C' completes all previous "DM" pairs to form "DMC".
                dmc_count += dm_count;
            }
            _ => {
                // Input is assumed valid per preconditions; undefined otherwise.
            }
        }
        prefix_ans[i + 1] = dmc_count;
    }

    let mut out = String::new();
    for _ in 0..q {
        let l: usize = it.next().unwrap().parse().unwrap();
        out.push_str(&format!("{}\n", prefix_ans[l]));
    }

    print!("{out}");
}
