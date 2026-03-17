fn main() {
    use std::io::{self, BufRead};

    const M: i64 = 998244353;
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which contains the count of subsequent integers
    let first_line = lines.next().unwrap().unwrap();
    let n: usize = first_line.trim().parse().expect("Expected a non-negative integer");

    let mut d = vec![0; 90310];
    let mut e = vec![0; 90310];
    let mut s = 0;
    let mut r = 1;

    // Process the subsequent integers
    for line in lines.take(n) {
        let a: i64 = line.unwrap().trim().parse().expect("Expected an integer");
        let mut t = s;

        while t != 0 {
            d[(t + a) as usize] = (d[t as usize] * 2) % M;
            e[(t + a) as usize] = e[t as usize] % M;
            t -= 1;
        }

        s += a;
        r = (r * 3) % M;
    }

    let mut t = s / 2;
    while t != 0 {
        r = (r - d[t as usize] * 3) % M;
        t -= 1;
    }

    // Adjust for the case when s is odd
    if s % 2 != 0 {
        r = (r + M - (e[(s / 2) as usize] * 3) % M) % M;
    }

    // Output the result
    println!("{}", r);
}
