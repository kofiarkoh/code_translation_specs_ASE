use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // No input; unspecified behavior. Exit quietly.
    };

    // Constraints: for each entity i, a list of (referenced_index_0_based, asserted_value_0_or_1).
    let mut cons: Vec<Vec<(usize, u8)>> = vec![Vec::new(); n];

    for i in 0..n {
        let m: usize = it.next().unwrap().parse().unwrap();
        let mut v = Vec::with_capacity(m);
        for _ in 0..m {
            let x_1based: usize = it.next().unwrap().parse().unwrap();
            let y: u8 = it.next().unwrap().parse().unwrap();
            // Preconditions guarantee 1 <= x <= n and y in {0,1}.
            v.push((x_1based - 1, y));
        }
        cons[i] = v;
    }

    if n == 0 {
        println!("0");
        return;
    }

    // Enumerate all subsets; n <= 20 so u32 is sufficient for bitmask.
    let total_masks: u32 = 1u32 << (n as u32);
    let mut best: u32 = 0;

    for mask in 0..total_masks {
        let mut ok = true;

        for i in 0..n {
            // Only constraints asserted by selected entities matter.
            if ((mask >> i) & 1) == 0 {
                continue;
            }
            for &(j, y) in &cons[i] {
                let selected_j = ((mask >> j) & 1) as u8;
                if selected_j != y {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }

        if ok {
            let cnt = mask.count_ones();
            if cnt > best {
                best = cnt;
            }
        }
    }

    println!("{}", best);
}
