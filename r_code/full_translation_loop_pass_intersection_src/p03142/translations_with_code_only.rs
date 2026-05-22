use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => std::process::exit(1),
    };
    let m: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => std::process::exit(1),
    };

    // C arrays are sized 114514 and indexed by node id (1..=N).
    // We'll keep the same capacity to preserve any implicit assumptions.
    const MAX: usize = 114_514;

    // Adjacency list: es[f] in C is a realloc-grown int*; in Rust use Vec<Vec<i32>>.
    let mut es: Vec<Vec<i32>> = vec![Vec::new(); MAX];
    let mut sansyo: Vec<i32> = vec![0; MAX];

    // Read edges: count is (N - 1 + M)
    let edge_count = n.saturating_sub(1) + m;
    for _ in 0..edge_count {
        let a: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => std::process::exit(1),
        };
        let b: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => std::process::exit(1),
        };
        // ae(A, B)
        es[a].push(b as i32);
        sansyo[b] += 1;
    }

    // next_buffer[2][MAX]
    let mut next_buffer: [Vec<i32>; 2] = [Vec::with_capacity(MAX), Vec::with_capacity(MAX)];
    let mut next_count = [0usize, 0usize];

    // answer[...], default 0 like C global
    let mut answer: Vec<i32> = vec![0; MAX];

    // Initialize phase 0 with nodes having sansyo == 0
    next_buffer[0].clear();
    for i in 1..=n {
        if sansyo[i] == 0 {
            next_buffer[0].push(i as i32);
        }
    }
    next_count[0] = next_buffer[0].len();

    let mut phase: usize = 0;
    while next_count[phase] > 0 {
        let other = 1 - phase;
        next_buffer[other].clear();
        next_count[other] = 0;

        // Iterate current frontier
        for idx in 0..next_count[phase] {
            let target = next_buffer[phase][idx] as usize;
            // for (j = 0; j < ec[target]; j++)
            for &to in es[target].iter() {
                let to_usize = to as usize;
                // if (--sansyo[to] == 0) { ... }
                sansyo[to_usize] -= 1;
                if sansyo[to_usize] == 0 {
                    answer[to_usize] = target as i32;
                    next_buffer[other].push(to);
                }
            }
        }

        next_count[other] = next_buffer[other].len();
        phase = other;
    }

    // Output: exactly N lines, each "answer[i]\n"
    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", answer[i]));
    }
    print!("{out}");
}
