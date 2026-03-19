use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse::<i32>().ok().map(|x| x as isize).filter(|&x| x >= 0).unwrap() as usize,
        None => std::process::exit(1),
    };
    let m: usize = match it.next() {
        Some(v) => v.parse::<i32>().ok().map(|x| x as isize).filter(|&x| x >= 0).unwrap() as usize,
        None => std::process::exit(1),
    };

    // Fixed-size arrays in C: 114514. We'll keep the same capacity.
    const CAP: usize = 114_514;

    // Adjacency lists: C uses realloc per node; in Rust use Vec<Vec<i32>>.
    let mut es: Vec<Vec<i32>> = vec![Vec::new(); CAP];
    let mut ec: Vec<usize> = vec![0; CAP]; // keep to mirror C's ec[] usage

    let mut sansyo: Vec<i32> = vec![0; CAP];
    let mut answer: Vec<i32> = vec![0; CAP];

    // Read edges: for i = 1; i <= N - 1 + M; i++
    let edge_count = n.saturating_sub(1).saturating_add(m);
    for _ in 0..edge_count {
        let a: i32 = match it.next() {
            Some(v) => v.parse().unwrap_or_else(|_| std::process::exit(1)),
            None => std::process::exit(1),
        };
        let b: i32 = match it.next() {
            Some(v) => v.parse().unwrap_or_else(|_| std::process::exit(1)),
            None => std::process::exit(1),
        };

        // C code assumes indices are within bounds; we mirror that assumption.
        let af = a as usize;
        let bt = b as usize;

        es[af].push(b);
        ec[af] += 1;
        sansyo[bt] += 1;
    }

    // next_buffer[2][CAP], next_count[2]
    let mut next_buffer: [Vec<i32>; 2] = [Vec::with_capacity(CAP), Vec::with_capacity(CAP)];
    let mut next_count: [usize; 2] = [0, 0];

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

        // Iterate over current frontier
        for idx in 0..next_count[phase] {
            let target = next_buffer[phase][idx] as usize;

            // for (j = 0; j < ec[target]; j++)
            // Use ec[target] to match C's loop bound (even though it equals es[target].len()).
            let out_deg = ec[target];
            for j in 0..out_deg {
                let to = es[target][j] as usize;
                sansyo[to] -= 1;
                if sansyo[to] == 0 {
                    answer[to] = target as i32;
                    next_buffer[other].push(to as i32);
                }
            }
        }

        next_count[other] = next_buffer[other].len();
        phase = other;
    }

    // Output: for i = 1; i <= N; i++ printf("%d\n", answer[i]);
    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", answer[i]));
    }
    print!("{out}");
}
