use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d %d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        let x_opt = it.next();
        if n_opt.is_none() || x_opt.is_none() {
            break; // No more complete pairs to read.
        }

        let n: i32 = n_opt.unwrap().parse().unwrap();
        let x: i32 = x_opt.unwrap().parse().unwrap();

        let mut b: i32 = 0;

        if n == 0 && x == 0 {
            break;
        }

        for k in 1..=n {
            for j in 1..=n {
                for i in 1..=n {
                    if i == j || j == k || i == k {
                        // do nothing
                    } else {
                        let a = i + j + k;
                        if a == x {
                            b += 1;
                        }
                    }
                }
            }
        }

        // In C, b is divisible by 6 due to counting permutations; integer division truncates.
        println!("{}", b / 6);
    }
}
