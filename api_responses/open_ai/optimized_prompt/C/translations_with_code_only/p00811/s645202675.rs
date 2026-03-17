use std::io::{self, BufRead};

const MAX: usize = 100000;

fn mpl(isprime: &mut [bool], pr: &mut Vec<usize>) -> usize {
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
    let handle = stdin.lock();
    let mut isprime = [false; MAX];
    let mut pr = Vec::new();
    let primenum = mpl(&mut isprime, &mut pr);

    for line in handle.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let m = nums.next().unwrap();
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();

        if (m | a | b) == 0 {
            break;
        }

        let mut p = 0;
        let mut q = 0;

        for i in 0..primenum {
            let mut l = i + 1; // Start l from i + 1
            let mut r = primenum - 1;

            while l <= r {
                let j = (l + r) / 2;
                if (pr[i] as i64) * (pr[j] as i64) <= (m as i64) &&
                   (a as i64) * (pr[j] as i64) <= (pr[i] as i64) * (b as i64) {
                    l = j + 1;
                } else {
                    r = j - 1;
                }
            }

            if l > i + 1 && (pr[i] as i64) * (pr[l - 1] as i64) <= (m as i64) &&
               (a as i64) * (pr[l - 1] as i64) <= (pr[i] as i64) * (b as i64) &&
               (pr[i] * pr[l - 1]) > (p * q) {
                p = pr[i];
                q = pr[l - 1];
            }
        }
        println!("{} {}", p, q);
    }
}
