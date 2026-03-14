use std::io::{self, BufRead};

#[derive(Debug)]
struct Hoge {
    n: i32,
    count: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut s: [Hoge; 13] = Default::default();
    let mut sn: usize;
    let mut seq: [i32; 13] = Default::default();

    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n == 0 {
            break;
        }

        let n = n as usize;
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        for (i, &num) in nums.iter().enumerate().take(n) {
            seq[i] = num;
        }

        let mut i = 0;
        loop {
            let mut fix = 1;
            sn = 0;

            for j in 0..n {
                let mut k = 0;
                while k < sn {
                    if s[k].n == seq[j] {
                        s[k].count += 1;
                        break;
                    }
                    k += 1;
                }
                if k == sn {
                    s[sn].n = seq[j];
                    s[sn].count = 1;
                    sn += 1;
                }
            }

            for j in 0..n {
                let mut k = 0;
                while k < sn {
                    if s[k].n == seq[j] {
                        if seq[j] != s[k].count {
                            fix = 0;
                        }
                        seq[j] = s[k].count;
                        break;
                    }
                    k += 1;
                }
            }

            if fix == 1 {
                break;
            }
            i += 1;
        }

        println!("{}", i);
        for j in 0..n {
            s[j].n = -1;
            s[j].count = -1;
            if j < n - 1 {
                print!("{} ", seq[j]);
            } else {
                println!("{}", seq[j]);
            }
        }
    }
}
