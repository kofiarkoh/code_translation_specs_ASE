use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // EOF
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break; // termination condition
        }

        // Spec bounds n <= 100; C code assumes it. We follow spec and input contract.
        let mut a: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            let v: i32 = it.next().unwrap().parse().unwrap();
            a.push(v);
        }

        // Bubble sort counting swaps, matching the C loops exactly.
        let mut swap_count: i32 = 0;
        if n >= 2 {
            for u in (1..n).rev() {
                for i in 0..u {
                    if a[i] > a[i + 1] {
                        a.swap(i, i + 1);
                        swap_count += 1;
                    }
                }
            }
        }

        out.push_str(&format!("{swap_count}\n"));
    }

    print!("{out}");
}
