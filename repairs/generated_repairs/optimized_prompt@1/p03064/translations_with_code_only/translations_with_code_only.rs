fn main() {
    const M: i64 = 998244353;
    let mut d = vec![0; 90310];
    let mut e = vec![0; 90310];
    let mut s = 0;
    let mut r = 1;

    // Read the first integer
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let first_value: i64 = input.trim().parse().unwrap();
    s += first_value;

    // Initialize d and e for the first value
    d[first_value as usize] = 1;

    // Read subsequent integers until EOF
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        let value: i64 = input.trim().parse().unwrap();
        s += value;

        let mut t = s;
        while t >= value {
            d[t as usize] = (d[t as usize] + d[(t - value) as usize]) % M;
            e[t as usize] = (e[t as usize] + e[(t - value) as usize]) % M;
            t -= 1;
        }
        r = (r * 3) % M;
    }

    let mut t = s / 2;
    while t >= 0 {
        r = (r - d[t as usize] * 3) % M;
        t -= 1;
    }

    // Handle negative modulo
    r = (r + M) % M;

    // Print the result
    println!("{}", (r - (if s % 2 == 1 { e[(s / 2) as usize] * 3 } else { 0 }) + M) % M);
}
