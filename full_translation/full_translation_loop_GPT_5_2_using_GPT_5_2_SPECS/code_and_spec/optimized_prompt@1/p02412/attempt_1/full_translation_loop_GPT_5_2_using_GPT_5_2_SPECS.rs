use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as integers (mirrors scanf-style whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        let x_opt = it.next();
        if n_opt.is_none() || x_opt.is_none() {
            // Spec: behavior on input exhaustion is not defined; we just stop.
            break;
        }

        let n: i32 = n_opt.unwrap().parse().unwrap();
        let x: i32 = x_opt.unwrap().parse().unwrap();

        if n == 0 && x == 0 {
            break;
        }

        // Count ordered triples (i, j, k) with distinct values in [1, n] summing to x,
        // then divide by 6 to get unordered triples (each unordered triple has 3! permutations).
        let mut b: i32 = 0;
        for k in 1..=n {
            for j in 1..=n {
                for i in 1..=n {
                    if i == j || j == k || i == k {
                        continue;
                    }
                    let a = i + j + k;
                    if a == x {
                        b += 1;
                    }
                }
            }
        }

        println!("{}", b / 6);
    }
}
