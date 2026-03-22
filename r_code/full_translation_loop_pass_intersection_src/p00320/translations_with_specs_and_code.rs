use std::io::{self, Read};
use std::process;

fn next_permutation(a: &mut [usize]) -> bool {
    // Lexicographic next permutation; returns false if already at last permutation.
    if a.len() < 2 {
        return false;
    }

    // Find rightmost i such that a[i] < a[i+1]
    let mut i_opt: Option<usize> = None;
    for i in (0..a.len() - 1).rev() {
        if a[i] < a[i + 1] {
            i_opt = Some(i);
            break;
        }
    }
    let i = match i_opt {
        Some(v) => v,
        None => return false,
    };

    // Reverse suffix a[i+1..]
    a[i + 1..].reverse();

    // Find first element in suffix greater than a[i] (suffix is now increasing)
    let pivot = a[i];
    let mut j = i + 1;
    while j < a.len() && a[j] <= pivot {
        j += 1;
    }
    // j must exist because there was an increasing pair before reversal.
    a.swap(i, j);
    true
}

fn main() {
    // Read all input; if fewer than 12 integers, exit non-zero with no guaranteed output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let mut hw = [[0i32; 2]; 6];
    for i in 0..6 {
        let a = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let b = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        hw[i][0] = a;
        hw[i][1] = b;
    }

    // Fixed adjacency constraints from the C code (source-of-truth for topology).
    // Each row: {pos1, side1, pos2, side2}, where side is 0/1 and may be XORed with rotation bit.
    const CONNECT: [[usize; 4]; 12] = [
        [0, 1, 1, 1],
        [0, 0, 2, 0],
        [0, 1, 3, 1],
        [0, 0, 4, 0],
        [1, 0, 2, 1],
        [2, 1, 3, 0],
        [3, 0, 4, 1],
        [4, 1, 1, 0],
        [5, 1, 1, 1],
        [5, 0, 4, 0],
        [5, 1, 3, 1],
        [5, 0, 4, 0],
    ];

    let mut perm: [usize; 6] = [0, 1, 2, 3, 4, 5];

    loop {
        for rotate in 0u32..(1u32 << 6) {
            let mut ok = true;
            for c in CONNECT.iter() {
                let p1 = c[0];
                let s1 = c[1];
                let p2 = c[2];
                let s2 = c[3];

                let r1 = ((rotate >> p1) & 1) as usize;
                let r2 = ((rotate >> p2) & 1) as usize;

                let len1 = hw[perm[p1]][s1 ^ r1];
                let len2 = hw[perm[p2]][s2 ^ r2];

                if len1 != len2 {
                    ok = false;
                    break;
                }
            }
            if ok {
                print!("yes\n");
                return;
            }
        }

        if !next_permutation(&mut perm) {
            break;
        }
    }

    print!("no\n");
}
