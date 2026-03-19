use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let mut p: i64 = iter.next().unwrap().parse().unwrap();
    let mut ans: i64 = 1;

    for i in 2..=((p as f64).sqrt() as i64) {
        let mut cnt = 0;
        while p % i == 0 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n {
            ans *= i.pow((cnt / n) as u32);
        }
    }
    
    if p > 1 && n == 1 {
        ans *= p;
    }

    println!("{}", ans);
}
