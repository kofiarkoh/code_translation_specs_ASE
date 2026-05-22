use std::io::{self, Read};
use std::process;

const CONNECT: [[usize; 4]; 12] = [
    // {pos, h/w index, pos, h/w index}
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
    [5, 0, 4, 0], // duplicated in the C code; preserved for exact semantics
];

fn next_permutation(a: &mut [usize]) -> bool {
    // Matches the C implementation's behavior: generates next lexicographic permutation.
    let n = a.len();
    if n < 2 {
        return false;
    }

    // Find rightmost i with a[i] < a[i+1]
    let mut i_opt: Option<usize> = None;
    for i in (0..=n - 2).rev() {
        if a[i] < a[i + 1] {
            i_opt = Some(i);
            break;
        }
    }
    let target = match i_opt {
        Some(t) => t,
        None => return false,
    };

    // Reverse suffix [target+1, n)
    let mut l = target + 1;
    let mut r = n - 1;
    while l < r {
        a.swap(l, r);
        l += 1;
        r -= 1;
    }

    // Find first element in suffix greater than a[target]
    let mut j = target + 1;
    while j < n && a[j] <= a[target] {
        j += 1;
    }
    // In valid next-permutation state, such j must exist.
    a.swap(target, j);
    true
}

fn main() {
    // Read all input; require exactly 12 integers (6 pairs). If not, exit non-zero and print nothing.
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

    let mut perm: [usize; 6] = [0, 1, 2, 3, 4, 5];

    loop {
        for rotate in 0u32..(1u32 << 6) {
            let mut ok = true;
            for c in CONNECT.iter() {
                let p1 = c[0];
                let idx1 = c[1] ^ (((rotate >> p1) & 1) as usize);
                let p2 = c[2];
                let idx2 = c[3] ^ (((rotate >> p2) & 1) as usize);

                let len1 = hw[perm[p1]][idx1];
                let len2 = hw[perm[p2]][idx2];
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
