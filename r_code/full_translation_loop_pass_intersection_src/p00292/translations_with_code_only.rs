use std::io::{self, Read};

fn main() {
    // Read entire stdin, like repeated scanf calls.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code uses implicit int globals: n, x; and main(y) where y is an int parameter.
    // We'll model y as a local i32 that gets assigned by scanning.
    let n0: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // If no input, do nothing (closest to failing scanf early).
    };

    let mut n = n0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    // for(scanf("%d",&n); n--; printf("%d\n",(x-1)%y+1)) scanf("%d%d",&x,&y);
    // Note: In C, the loop condition is `n--` (post-decrement): it tests the old value.
    // So it runs exactly n0 times if n0 > 0.
    while n != 0 {
        n -= 1;

        // Body: scanf("%d%d",&x,&y);
        // If input is missing, C would leave x/y unchanged and still proceed; we mimic by
        // stopping if we can't read both (best-effort without UB).
        let xs = match it.next() {
            Some(s) => s,
            None => break,
        };
        let ys = match it.next() {
            Some(s) => s,
            None => break,
        };
        x = xs.parse().unwrap();
        y = ys.parse().unwrap();

        // Increment expression: printf("%d\n",(x-1)%y+1)
        // C's `%` and division by zero are UB; Rust would panic on y==0.
        // We emulate C's behavior as closely as possible by avoiding a Rust panic and
        // producing no defined output in that case (since C has no defined result).
        if y == 0 {
            // Undefined in C; choose to stop to avoid panicking.
            break;
        }

        // Use wrapping arithmetic to emulate C signed overflow behavior on typical
        // two's-complement machines (C overflow is UB, but this is the closest intent).
        let xm1 = x.wrapping_sub(1);
        let r = xm1 % y; // Rust remainder matches C's sign rule (trunc toward zero).
        let out = r.wrapping_add(1);

        print!("{}\n", out);
    }

    // C calls exit(0);
}
