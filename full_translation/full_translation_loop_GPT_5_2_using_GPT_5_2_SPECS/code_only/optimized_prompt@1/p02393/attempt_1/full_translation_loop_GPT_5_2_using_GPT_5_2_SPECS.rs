use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // The C code does not check scanf's return value; if input is missing,
    // behavior would be undefined. Here we unwrap to fail similarly (panic).
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();

    // Determine d,e,f according to the same chain of comparisons.
    // This effectively sorts a,b,c in nondecreasing order.
    let (d, e, f): (i32, i32, i32) = if a <= b && b <= c {
        (a, b, c)
    } else if a <= c && c <= b {
        (a, c, b)
    } else if b <= a && a <= c {
        (b, a, c)
    } else if b <= c && c <= a {
        (b, c, a)
    } else if c <= a && a <= b {
        (c, a, b)
    } else if c <= b && b <= a {
        (c, b, a)
    } else {
        // For total order on i32, one of the above must hold.
        unreachable!();
    };

    // Match C's printf formatting exactly: three ints separated by spaces, then newline.
    print!("{} {} {}\n", d, e, f);
}
