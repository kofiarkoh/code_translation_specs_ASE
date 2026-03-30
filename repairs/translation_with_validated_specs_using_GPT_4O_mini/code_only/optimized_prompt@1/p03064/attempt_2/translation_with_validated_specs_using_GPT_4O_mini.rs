use std::io::{self, Write};

const M: i64 = 998244353;
const SIZE: usize = 90310;

fn main() {
    let mut d = vec![0; SIZE];
    let mut e = vec![0; SIZE];
    let mut s = 0;
    let mut r = 1;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    // Read the first value into d[0] and e[0]
    if let Some(a) = iter.next() {
        d[0] = a;
        e[0] = a;
    }

    while let Some(a) = iter.next() {
        let mut t = s;
        while t >= 0 {
            let t_usize = t as usize;
            d[t_usize + a as usize] = (d[t_usize + a as usize] + d[t_usize] * 2 % M) % M;
            e[t_usize + a as usize] = (e[t_usize + a as usize] + e[t_usize]) % M;
            t -= 1;
        }
        s += a;
        r = r * 3 % M;
    }

    let mut t = s / 2;
    while t >= 0 {
        r = (r - d[t as usize] * 3) % M;
        t -= 1;
    }

    // Adjust for negative values in Rust
    r = (r + M) % M;

    // Print the result
    println!("{}", r);
    io::stdout().flush().unwrap();
}
