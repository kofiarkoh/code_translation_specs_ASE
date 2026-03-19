use std::io::{self, Read};

fn main() {
    // Read entire stdin; input is assumed well-formed per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code loops: while (scanf("%lld",&n), n) { ... }
    // If n is missing, terminate silently.
    while let Some(n_tok) = it.next() {
        let mut n: i64 = n_tok.parse().unwrap();
        if n == 0 {
            break;
        }
        // Spec says n is positive; if non-positive, terminate without output.
        if n < 0 {
            return;
        }

        let mut a = [[false; 25]; 25];
        for _ in 0..n {
            let x: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
            let y: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
            a[x][y] = true;
        }

        let m: i64 = it.next().unwrap().parse().unwrap();
        // Spec says m is positive; if non-positive, terminate without output.
        if m <= 0 {
            return;
        }

        let mut px: i64 = 10;
        let mut py: i64 = 10;

        if a[px as usize][py as usize] {
            a[px as usize][py as usize] = false;
            n -= 1;
        }

        // Direction deltas match C's d4 array usage:
        // d4 = [0,1,0,-1,0]; px += d4[d]; py += d4[d+1]
        let d4: [i64; 5] = [0, 1, 0, -1, 0];

        for _ in 0..m {
            let c = it.next().unwrap().as_bytes()[0] as char;
            let q: i64 = it.next().unwrap().parse().unwrap();

            let d: usize = match c {
                'N' => 0,
                'E' => 1,
                'S' => 2,
                _ => 3, // matches C's final else
            };

            for _ in 0..q {
                px += d4[d];
                py += d4[d + 1];

                // Behavior is unspecified if out of bounds; C would have UB.
                // We assume inputs keep positions within 0..=24 as per spec preconditions.
                let ux = px as usize;
                let uy = py as usize;

                if a[ux][uy] {
                    a[ux][uy] = false;
                    n -= 1;
                }
            }
        }

        if n != 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
