use std::io::{self, Write};

const M: i64 = 998244353;
const SIZE: usize = 90310;

fn main() {
    let mut d = vec![0; SIZE];
    let mut e = vec![0; SIZE];
    let mut s: i64 = 0; // Change s to i64 to match the type of a
    let mut r: i64 = 1; // Change r to i64 to match the type of M

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut a: i64 = input.trim().parse().unwrap();
    
    // Read the next integer until EOF
    while let Ok(_) = io::stdin().read_line(&mut input) {
        if input.trim().is_empty() {
            break;
        }
        let a_input: i64 = input.trim().parse().unwrap();
        a = a_input;

        let mut t = s;
        while t >= 0 {
            d[(t + a) as usize % SIZE] = (d[t as usize] * 2) % M;
            e[(t + a) as usize % SIZE] = e[t as usize] % M;
            t -= 1;
        }
        s += a;
        r = (r * 3) % M;
    }

    let mut t = s / 2;
    while t >= 0 {
        r = (r - d[t as usize] * 3) % M;
        t -= 1;
    }

    // Handle negative modulo
    if r < 0 {
        r += M;
    }

    // Handle the case for s being odd
    if s % 2 != 0 {
        r = (r - e[(s / 2) as usize] * 3 + M) % M;
    }

    // Print the result
    print!("{}", r);
    io::stdout().flush().unwrap();
}
