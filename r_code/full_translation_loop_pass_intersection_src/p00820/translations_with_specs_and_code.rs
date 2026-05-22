use std::io::{self, Read};

fn solve(n: i32, c: i32, low: i32) -> i32 {
    let mut ret = 0i32;
    let mut i = low;

    // Mirrors C loop: for (i=low; i*i<=n; i++)
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
    // Read all input; if parsing fails, terminate without further output (per spec).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut out = String::new();
    for tok in input.split_whitespace() {
        let n: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => return, // input read failure => terminate without further output
        };
        if n == 0 {
            break; // terminating value
        }
        let ret = solve(n, 1, 1);
        out.push_str(&format!("{ret}\n"));
    }

    print!("{out}");
}
