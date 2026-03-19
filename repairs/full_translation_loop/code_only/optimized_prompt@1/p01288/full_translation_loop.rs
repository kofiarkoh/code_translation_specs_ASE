use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    while input.read_line(&mut buffer).unwrap() > 0 {
        let mut iter = buffer.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut p = vec![0; n + 1];
        let mut m = vec![0; n + 1];
        let mut v = vec![0; q];
        let mut q_chars = vec![' '; q];
        let mut r: i64 = 0;

        for i in 0..n {
            p[i] = i;
            m[i] = 0;
        }
        m[1] = 1;

        for i in 2..=n {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            let t: usize = buffer.trim().parse().unwrap();
            p[i] = t;
        }

        for i in 0..q {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            let parts: Vec<&str> = buffer.split_whitespace().collect();
            q_chars[i] = parts[0].chars().next().unwrap();
            v[i] = parts[1].parse().unwrap();
            if q_chars[i] == 'M' {
                m[v[i]] += 1;
            }
        }

        for i in (0..q).rev() {
            if q_chars[i] == 'M' {
                m[v[i]] -= 1;
            } else {
                let mut t = v[i];
                while m[t] == 0 {
                    t = p[t];
                }
                r += t as i64;
            }
        }

        println!("{}", r);
        buffer.clear();
    }
}
