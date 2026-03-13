use std::io::{self, Read};

fn main() {
    // Read all stdin as whitespace-separated tokens, similar to scanf("%d", ...) then repeated "%lf".
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    // Parse n as a signed 32-bit int to mirror C's `int`.
    let n: i32 = match toks.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // In C: for (i=0; i<n; i++) { ... }
    // If n <= 0 the loop does zero iterations. Preserve that behavior.
    if n <= 0 {
        return;
    }

    for _ in 0..(n as usize) {
        // Parse eight f64 values in the same order as the C code's scanf.
        // We use unwrap() to reflect the C program's assumption of correct input.
        let x1: f64 = toks.next().unwrap().parse().unwrap();
        let y1: f64 = toks.next().unwrap().parse().unwrap();
        let x2: f64 = toks.next().unwrap().parse().unwrap();
        let y2: f64 = toks.next().unwrap().parse().unwrap();
        let x3: f64 = toks.next().unwrap().parse().unwrap();
        let y3: f64 = toks.next().unwrap().parse().unwrap();
        let x4: f64 = toks.next().unwrap().parse().unwrap();
        let y4: f64 = toks.next().unwrap().parse().unwrap();

        // Perform divisions with f64 to preserve IEEE-754 behavior (Inf, -Inf, NaN),
        // matching the C semantics for double division and comparison.
        let sl1 = (y2 - y1) / (x2 - x1);
        let sl2 = (y4 - y3) / (x4 - x3);

        if sl1 == sl2 {
            // puts("YES") in C prints the string followed by a newline.
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
