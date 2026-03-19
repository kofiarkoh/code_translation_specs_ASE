use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut outputs = Vec::new();

    loop {
        let n_opt = it.next();
        let x_opt = it.next();
        if n_opt.is_none() || x_opt.is_none() {
            break;
        }

        let n: i64 = n_opt.unwrap().parse().unwrap();
        let x: i64 = x_opt.unwrap().parse().unwrap();

        if n == 0 && x == 0 {
            break;
        }

        // Count ordered triples (i, j, k) with distinct values in [1..=n] summing to x,
        // then divide by 6 to get unique combinations (unordered triples).
        let mut count: i64 = 0;
        if n >= 1 {
            for i in 1..=n {
                for j in 1..=n {
                    if j == i {
                        continue;
                    }
                    for k in 1..=n {
                        if k == i || k == j {
                            continue;
                        }
                        if i + j + k == x {
                            count += 1;
                        }
                    }
                }
            }
        }

        outputs.push((count / 6).to_string());
    }

    print!("{}", outputs.join("\n"));
}
