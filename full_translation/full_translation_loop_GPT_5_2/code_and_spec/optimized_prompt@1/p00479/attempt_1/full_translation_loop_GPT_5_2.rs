use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (matches scanf behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse().unwrap();

    let mut out = String::new();

    for _ in 0..k {
        let mut ai: i32 = it.next().unwrap().parse().unwrap();
        let mut bi: i32 = it.next().unwrap().parse().unwrap();

        // Adjust each value to min(x, n+1-x)
        if n + 1 - ai < ai {
            ai = n + 1 - ai;
        }
        if n + 1 - bi < bi {
            bi = n + 1 - bi;
        }

        // Compute c in range 1..=3
        let c: i32 = if ai > bi {
            (bi - 1).rem_euclid(3) + 1
        } else {
            (ai - 1).rem_euclid(3) + 1
        };

        out.push_str(&format!("{c}\n"));
    }

    print!("{out}");
}
