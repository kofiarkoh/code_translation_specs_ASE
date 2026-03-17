fn main() {
    use std::io::{self, BufRead};

    const M: i64 = 998244353;
    let mut d = vec![0; 90310];
    let mut e = vec![0; 90310];
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut inputs = handle.lines().filter_map(Result::ok).filter_map(|line| line.parse::<i64>().ok);

    // Read the first input to initialize d[0] and e[0]
    let first_input = match inputs.next() {
        Some(value) if value >= 0 => value,
        _ => return, // Terminate if no valid first input
    };

    d[0] = first_input;
    e[0] = first_input;

    let mut s = first_input;
    let mut r = 1;

    for a in inputs {
        let mut t = s;
        while t >= 0 {
            let t_usize = t as usize; // Safe conversion since t is non-negative
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

    // Adjust for the case where s is odd
    if s % 2 != 0 {
        r = (r + M - e[(s / 2) as usize] * 3 % M) % M;
    }

    println!("{}", r);
}
