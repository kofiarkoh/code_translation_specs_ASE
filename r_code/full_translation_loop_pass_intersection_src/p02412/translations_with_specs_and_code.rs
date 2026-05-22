use std::io::{self, Read};

fn main() {
    // Read entire stdin; spec defines no special error handling for malformed input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        let x_opt = it.next();
        if n_opt.is_none() || x_opt.is_none() {
            // Spec does not define behavior for missing input; just stop.
            break;
        }

        let n: i32 = n_opt.unwrap().parse().unwrap();
        let x: i32 = x_opt.unwrap().parse().unwrap();

        if n == 0 && x == 0 {
            break;
        }

        // Invariant per spec: count ordered permutations; divide by 6 for unordered triples.
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
