use std::io::{self, Read};

fn solve(n: i32, c: i32, low: i32) -> i32 {
    let mut ret: i32 = 0;
    let mut i: i32 = low;

    // Mirror C loop condition: for(i=low; i*i<=n; i++)
    // Use i64 for the multiplication to avoid Rust overflow panics while
    // preserving the intended arithmetic comparison.
    while (i as i64) * (i as i64) <= n as i64 {
        let sq = i * i; // safe here because i*i <= n and n is i32
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
    // Read entire stdin, then parse ints similarly to repeated scanf("%d", &n).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let mut out = String::new();

    while let Some(tok) = it.next() {
        let n: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // emulate scanf failure: stop processing
        };
        if n == 0 {
            break;
        }
        let ret = solve(n, 1, 1);
        out.push_str(&format!("{}\n", ret));
    }

    print!("{}", out);
}
