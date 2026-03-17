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
            let mut coords = String::new();
            reader.read_line(&mut coords).unwrap();
            let mut parts = coords.split_whitespace();
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();
            a[x][y] = 1;
        }

        let mut m_line = String::new();
        reader.read_line(&mut m_line).unwrap();
        let m: i64 = m_line.trim().parse().unwrap();

        let mut px = 10;
        let mut py = 10;
        if a[px][py] == 1 {
            a[px][py] = 0;
            n -= 1;
        }

        for _ in 0..m {
            let mut command = String::new();
            reader.read_line(&mut command).unwrap();
            let mut parts = command.split_whitespace();
            let c: char = parts.next().unwrap().chars().next().unwrap();
            let q: i64 = parts.next().unwrap().parse().unwrap();

            let d = match c {
                'N' => 0,
                'E' => 1,
                'S' => 2,
                'W' => 3,
                _ => unreachable!(), // This should not happen based on the input specification
            };

            for _ in 0..q {
                px += D4[d];
                py += D4[d + 1];
                if a[px][py] == 1 {
                    a[px][py] = 0;
                    n -= 1;
                }
            }
        }

        println!("{}", if n > 0 { "No" } else { "Yes" });
    }
}
