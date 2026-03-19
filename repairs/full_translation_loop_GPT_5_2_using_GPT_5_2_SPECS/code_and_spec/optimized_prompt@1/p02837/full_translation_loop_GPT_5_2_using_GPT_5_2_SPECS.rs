use std::io::{self, Read};

fn popcount(mut z: u32) -> u32 {
    // Mirrors the C recursive counter(z>>1)+(z&1), but iteratively.
    let mut c = 0;
    while z != 0 {
        c += z & 1;
        z >>= 1;
    }
    c
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // No input; unspecified, but exit quietly.
    };

    // Constraints per entity: for each entity i, store list of (ref_index_0based, asserted_value).
    let mut constraints: Vec<Vec<(usize, u8)>> = vec![Vec::new(); n];

    for i in 0..n {
        let a_i: usize = it.next().unwrap().parse().unwrap();
        let mut v = Vec::with_capacity(a_i);
        for _ in 0..a_i {
            let x: usize = it.next().unwrap().parse().unwrap(); // 1-based in input
            let y: u8 = it.next().unwrap().parse().unwrap(); // 0/1
            v.push((x - 1, y));
        }
        constraints[i] = v;
    }

    let mut best: u32 = 0;

    // Enumerate all subsets.
    // Preconditions guarantee n <= 20, so 1<<n fits in u32.
    let total = 1u32 << (n as u32);
    for mask in 0..total {
        let mut ok = true;

        for j in 0..n {
            if (mask & (1u32 << (j as u32))) == 0 {
                continue; // constraints of non-selected entities do not matter
            }
            for &(ref_idx, asserted) in &constraints[j] {
                let bit = ((mask >> (ref_idx as u32)) & 1) as u8;
                // C condition: ((mask>>(x-1)&1) ^ y) != 0 => contradiction
                if (bit ^ asserted) != 0 {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }

        if ok {
            let cnt = popcount(mask);
            if cnt > best {
                best = cnt;
            }
        }
    }

    // Preserve C output formatting: no trailing newline.
    print!("{}", best);
}
