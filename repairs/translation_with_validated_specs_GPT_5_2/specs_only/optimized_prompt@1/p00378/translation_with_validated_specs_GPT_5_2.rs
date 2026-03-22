use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let x: i64 = it.next().unwrap().parse().unwrap();

    let ans: i64;

    if a >= 2 * b {
        // B * ceil(X / 500)
        let n = (x + 500 - 1) / 500;
        ans = b * n;
    } else {
        let q = x / 1000;
        let r = x % 1000;

        let mut total = q * a;
        if r != 0 {
            if r <= 500 && b < a {
                total += b;
            } else {
                total += a;
            }
        }
        ans = total;
    }

    println!("{ans}");
}
