use std::io::{self, BufRead};

const MAX: usize = 100_000;

fn mpl(isprime: &mut [bool; MAX], pr: &mut Vec<usize>) -> usize {
    for i in 0..MAX {
        isprime[i] = true;
    }
    isprime[0] = false;
    isprime[1] = false;
    let mut primenum = 0;

    for i in 2..MAX {
        if isprime[i] {
            pr.push(i);
            primenum += 1;
            let mut j = i + i;
            while j < MAX {
                isprime[j] = false;
                j += i;
            }
        }
    }
    primenum
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut isprime = [true; MAX];
    let mut pr = Vec::new();
    let primenum = mpl(&mut isprime, &mut pr);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let m: i64 = parts.next().unwrap().parse().unwrap();
        let a: i64 = parts.next().unwrap().parse().unwrap();
        let b: i64 = parts.next().unwrap().parse().unwrap();

        if m | a | b == 0 {
            break;
        }

        let mut p = 0;
        let mut q = 0;

        for i in 0..primenum {
            let mut l = i + 1; // Start l from i + 1
            let mut r = primenum - 1;

            while l <= r {
                let j = (l + r) / 2;
                if (pr[i] as i64) * (pr[j] as i64) <= m && (a * (pr[j] as i64)) <= (pr[i] as i64) * b {
                    l = j + 1;
                } else {
                    r = j - 1;
                }
            }

            if l > i + 1 && (pr[i] as i64) * (pr[l - 1] as i64) <= m && (a * (pr[l - 1] as i64)) <= (pr[i] as i64) * b && (pr[i] * pr[l - 1]) > (p * q) {
                p = pr[i];
                q = pr[l - 1];
            }
        }
        println!("{} {}", p, q);
    }
}
