use std::io::{self, Read};

fn solve(n: i32, c: i32, low: i32) -> i32 {
    let mut ret = 0i32;
    let mut i = low;

    // Mirror C loop condition: i*i <= n (with i32 arithmetic).
    // Typical AOJ inputs keep this within range; spec does not define overflow behavior.
    while i * i <= n {
        let sq = i * i;
        if n == sq {
            ret += 1;
        } else if c < 4 {
            ret += solve(n - sq, c + 1, i);
        }
        i += 1;
    }
    ret
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();
    for tok in input.split_whitespace() {
        let n: i32 = tok.parse().unwrap();
        if n == 0 {
            break;
        }
        let ret = solve(n, 1, 1);
        out.push_str(&format!("{ret}\n"));
    }

    print!("{out}");
}
