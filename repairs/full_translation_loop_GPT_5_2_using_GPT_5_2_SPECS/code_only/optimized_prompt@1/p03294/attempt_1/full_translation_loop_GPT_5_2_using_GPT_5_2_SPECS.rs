use std::io::{self, Read};

fn main() {
    // Read entire stdin, like repeated scanf calls would consume whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In C, if scanf fails, N would be uninitialized (UB). Here we assume valid input.
    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // C uses a VLA `int a[N];` but only uses it to store inputs; we can avoid storing.
    // Use i32 to match typical C `int` width; use wrapping arithmetic to emulate C overflow behavior.
    let mut ans: i32 = 0;

    for _ in 0..n {
        let x: i32 = it.next().unwrap().parse().unwrap();
        ans = ans.wrapping_add(x.wrapping_sub(1));
    }

    // C prints without a trailing newline.
    print!("{}", ans);
}
