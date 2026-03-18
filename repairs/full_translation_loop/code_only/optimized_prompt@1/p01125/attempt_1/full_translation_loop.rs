use std::io::{self, BufRead};

const D4: [isize; 5] = [0, 1, 0, -1, 0];

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        let mut n_input = String::new();
        reader.read_line(&mut n_input).unwrap();
        let n: i64 = n_input.trim().parse().unwrap();
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

        let mut m_input = String::new();
        reader.read_line(&mut m_input).unwrap();
        let m: i64 = m_input.trim().parse().unwrap();

        let mut px: isize = 10;
        let mut py: isize = 10;
        if a[px as usize][py as usize] == 1 {
            a[px as usize][py as usize] = 0;
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
                _ => unreachable!(),
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
