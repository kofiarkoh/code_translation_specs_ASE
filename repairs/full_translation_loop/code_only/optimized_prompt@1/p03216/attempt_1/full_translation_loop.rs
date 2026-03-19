use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut n = String::new();
    handle.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut s = String::new();
    handle.read_line(&mut s).unwrap();
    let s: Vec<char> = s.trim().chars().collect();

    let mut q = String::new();
    handle.read_line(&mut q).unwrap();
    let q: usize = q.trim().parse().unwrap();

    let mut k = vec![0; 102];
    for i in 0..q {
        let mut temp = String::new();
        handle.read_line(&mut temp).unwrap();
        k[i] = temp.trim().parse().unwrap();
    }

    for j in 0..q {
        let mut ans: i64 = 0;
        let mut d: i64 = 0;
        let mut m: i64 = 0;
        let mut dm: i64 = 0;

        for i in 0..k[j] {
            if s[i] == 'D' {
                d += 1;
            }
            if s[i] == 'M' {
                m += 1;
                dm += d;
            }
            if s[i] == 'C' {
                ans += dm;
            }
        }

        for i in k[j]..n {
            if s[i - k[j]] == 'D' {
                dm -= m;
                d -= 1;
            }
            if s[i - k[j]] == 'M' {
                m -= 1;
            }
            if s[i] == 'D' {
                d += 1;
            }
            if s[i] == 'M' {
                m += 1;
                dm += d;
            }
            if s[i] == 'C' {
                ans += dm;
            }
        }

        println!("{}", ans);
    }
}
