use std::io;

fn func(s: &mut [u8], n: usize, m: usize) -> i32 {
    let mut sum = 0;
    let mut p = 0;

    for i in 0..n {
        if m & (1 << i) != 0 {
            let t = s[i + 1];
            s[i + 1] = 0; // Null-terminate the string
            sum += std::str::from_utf8(&s[p..]).unwrap_or("").parse::<i32>().unwrap_or(0);
            s[i + 1] = t; // Restore the original character
            p = i + 1;
        }
    }
    sum + std::str::from_utf8(&s[p..]).unwrap_or("").parse::<i32>().unwrap_or(0)
}

fn main() {
    let mut input = String::new();
    let mut paper = vec![0u8; 10];

    loop {
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            break; // Exit on read error
        }
        let mut parts = input.split_whitespace();
        let target: i32 = match parts.next() {
            Some(part) => part.parse().unwrap_or(0),
            None => break,
        };
        let paper_str = match parts.next() {
            Some(part) => part,
            None => break,
        };

        if target == 0 {
            break;
        }

        let n = paper_str.len();
        paper[..n].copy_from_slice(paper_str.as_bytes());
        let mut res = 0;
        let mut best = -1;
        let mut cnt = 0;

        for i in (1 << (n - 1))..(1 << n) {
            let t = func(&mut paper, n, i);
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
            if cnt == 0 {
                println!("error");
            } else {
                println!("rejected");
            }
        } else {
            print!("{} ", best);
            for i in 0..n {
                print!("{}", paper[i] as char);
                if res & (1 << i) != 0 {
                    if i == n - 1 {
                        println!();
                    } else {
                        print!(" ");
                    }
                }
            }
        }
    }
}
