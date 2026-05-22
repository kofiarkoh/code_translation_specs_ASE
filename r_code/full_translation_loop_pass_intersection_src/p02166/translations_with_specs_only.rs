use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: i64 = it.next().unwrap().parse().unwrap();

    let n_usize = n as usize;
    let mut a: Vec<i64> = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        a.push(it.next().unwrap().parse().unwrap());
    }

    if k == n {
        // Check if mapping is a uniform cyclic shift:
        // exists x such that for all i (1-based), a[i] - i ≡ x (mod n).
        if n == 0 {
            // Not specified; keep narrow behavior: with no positions, accept vacuously.
            println!("Yes");
            return;
        }
        let x = a[0] - 1;
        for (idx, &val) in a.iter().enumerate() {
            let i = (idx as i64) + 1;
            let diff = val - i;
            // Use Euclidean remainder to model congruence modulo n for possibly negative values.
            if (diff - x).rem_euclid(n) != 0 {
                println!("No");
                return;
            }
        }
        println!("Yes");
    } else {
        // Treat as functional graph; count number of directed cycles/components (cnt).
        // Mutate a[] by overwriting visited nodes with sentinel -1.
        let mut cnt: i64 = 0;
        for i in 0..n_usize {
            if a[i] != -1 {
                cnt += 1;
                let mut x = i;
                while a[x] != -1 {
                    let y = (a[x] - 1) as usize; // 1-based to 0-based; precondition assumes valid.
                    a[x] = -1;
                    x = y;
                }
            }
        }

        if ((n - cnt) & 1) == 1 && (k & 1) == 1 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
