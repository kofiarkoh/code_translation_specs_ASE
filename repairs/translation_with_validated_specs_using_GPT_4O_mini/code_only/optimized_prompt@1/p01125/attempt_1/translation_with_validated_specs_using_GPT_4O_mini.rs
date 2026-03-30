use std::io::{self, BufRead};

const D4: [i64; 5] = [0, 1, 0, -1, 0];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let n: i64 = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut a = [[0; 25]; 25];
        for _ in 0..n {
            let coords: Vec<i64> = reader.lines().next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let x = coords[0];
            let y = coords[1];
            a[x as usize][y as usize] = 1;
        }

        let m: i64 = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
        let mut px = 10;
        let mut py = 10;

        if a[px as usize][py as usize] == 1 {
            a[px as usize][py as usize] = 0;
            n -= 1;
        }

        for _ in 0..m {
            let command: Vec<String> = reader.lines().next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let c = &command[0];
            let q: i64 = command[1].parse().unwrap();
            let d = match c.as_str() {
                "N" => 0,
                "E" => 1,
                "S" => 2,
                _ => 3,
            };

            for _ in 0..q {
                px += D4[d];
                py += D4[d + 1];
                if a[px as usize][py as usize] == 1 {
                    a[px as usize][py as usize] = 0;
                    n -= 1;
                }
            }
        }

        println!("{}", if n > 0 { "No" } else { "Yes" });
    }
}
