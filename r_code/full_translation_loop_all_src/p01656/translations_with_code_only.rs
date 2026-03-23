use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style tokenization).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // No input: C would have undefined behavior; we just exit.
    };
    let q: i32 = it.next().unwrap().parse().unwrap();

    // C: char names[50][32] = {"kogakubu10gokan"};
    // We'll store up to 50 strings; index 0 initialized, others empty.
    let mut names: Vec<String> = vec![String::new(); 50];
    names[0] = "kogakubu10gokan".to_string();

    // C: char* answer = names[0];
    // Track the index of the chosen answer.
    let mut answer_idx: usize = 0;

    // C loop: for(i=1;i<=N;i++)
    // Note: C would overflow the fixed array if N >= 50; we clamp to avoid panic.
    // This is a best-effort emulation without invoking UB in Rust.
    for i in 1..=n {
        let year: i32 = it.next().unwrap().parse().unwrap();
        let name = it.next().unwrap().to_string();

        if i < names.len() {
            names[i] = name;
            if year <= q {
                answer_idx = i;
            }
        } else {
            // If out of bounds, C would write past the array (UB).
            // We still need to consume input tokens; selection can't safely refer to storage.
            // So we ignore storing and do not update answer_idx.
        }
    }

    // C puts() prints the string followed by '\n'.
    print!("{}\n", names[answer_idx]);
}
