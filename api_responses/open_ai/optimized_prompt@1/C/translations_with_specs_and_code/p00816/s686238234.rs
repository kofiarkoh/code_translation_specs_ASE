use std::io::{self, Write};

fn func(s: &str, n: usize, m: usize) -> i32 {
    let mut sum = 0;
    let mut p = s.as_bytes();
    for i in 0..n {
        if m & (1 << i) != 0 {
            let t = p[i + 1];
            p[i + 1] = 0; // Temporarily null-terminate the string
            sum += p[0..=i].iter().map(|&c| (c - b'0') as i32).sum::<i32>(); // Convert substring to integer
            p[i + 1] = t; // Restore the original character
        }
    }
    sum + p.iter().map(|&c| (c - b'0') as i32).sum::<i32>() // Add the last part
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let mut target = String::new();
        let mut paper = String::new();

        // Read target and paper
        if stdin.read_line(&mut target).is_err() || stdin.read_line(&mut paper).is_err() {
            break;
        }

        let target: i32 = match target.trim().parse() {
            Ok(num) if num > 0 => num,
            _ => {
                println!("error");
                continue;
            }
        };

        let n = paper.trim().len();
        if n == 0 || n > 10 {
            println!("error");
            continue;
        }

        let mut res = 0;
        let mut best = -1;
        let mut cnt = 0;

        for i in (1 << (n - 1))..(1 << n) {
            let t = func(&paper.trim(), n, i);
            if t <= target {
                if best < t {
                    best = t;
                    res = i;
                    cnt = 1;
                } else if best == t {
                    cnt += 1;
                }
            }
        }

        if cnt != 1 {
            println!("{}", if cnt == 0 { "error" } else { "rejected" });
        } else {
            print!("{} ", best);
            for i in 0..n {
                print!("{}", paper.chars().nth(i).unwrap());
                if res & (1 << i) != 0 {
                    print!("{}", if i == n - 1 { '\n' } else { ' ' });
                }
            }
        }
    }
}
