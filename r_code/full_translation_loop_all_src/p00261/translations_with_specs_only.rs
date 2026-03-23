use std::io::{self, Read};

fn main() {
    // Transition table: states S0..S5 => 0..5, symbols 0/1 => 0/1.
    // None represents an undefined transition, which causes early stop.
    const TRANS: [[Option<usize>; 2]; 6] = [
        // S0
        [Some(1), Some(2)],
        // S1
        [None, Some(3)],
        // S2
        [Some(1), None],
        // S3
        [Some(4), Some(5)],
        // S4
        [Some(5), Some(2)],
        // S5
        [Some(2), Some(1)],
    ];

    const INITIAL: usize = 0; // S0
    const ACCEPTING: usize = 5; // S5

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for tok in input.split_whitespace() {
        // Termination token: first character is '#'
        if tok.as_bytes().first() == Some(&b'#') {
            break;
        }

        // The specification assumes tokens contain only '0'/'1' and fit in a fixed buffer.
        // If other characters appear, behavior is undefined; we map them to an out-of-range
        // symbol index to mimic "may index outside the transition table" by panicking.
        let mut state = INITIAL;

        for &b in tok.as_bytes() {
            let sym = match b {
                b'0' => 0usize,
                b'1' => 1usize,
                _ => 2usize, // will panic on indexing, reflecting undefined behavior
            };

            let next = TRANS[state][sym];
            match next {
                Some(ns) => state = ns,
                None => break, // undefined transition: stop early, keep current state
            }
        }

        if state == ACCEPTING {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
