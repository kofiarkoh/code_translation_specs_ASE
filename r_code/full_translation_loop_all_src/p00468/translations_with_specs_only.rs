use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        // Memory proportional to n^2 as per constraints.
        let mut adj = vec![vec![false; n + 1]; n + 1];

        for _ in 0..m {
            let a: usize = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            if a <= n && b <= n {
                adj[a][b] = true;
                adj[b][a] = true;
            } else {
                // Preconditions say identifiers are intended to be within 1..n.
                // If violated, behavior is unspecified; we avoid panicking here.
            }
        }

        let mut within_two = vec![false; n + 1];

        // Direct neighbors of 1.
        for x in 2..=n {
            if adj[1][x] {
                within_two[x] = true;
            }
        }

        // Neighbors of neighbors: 1 - y - x, where y is directly connected to 1.
        for y in 2..=n {
            if adj[1][y] {
                for x in 2..=n {
                    if adj[y][x] {
                        within_two[x] = true;
                    }
                }
            }
        }

        // Never count individual 1.
        within_two[1] = false;

        let mut count = 0usize;
        for x in 2..=n {
            if within_two[x] {
                count += 1;
            }
        }

        out.push_str(&format!("{count}\n"));
    }

    print!("{out}");
}
