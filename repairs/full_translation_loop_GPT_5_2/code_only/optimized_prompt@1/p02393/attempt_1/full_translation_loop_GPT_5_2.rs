use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume formatted input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // The C code assumes three ints are successfully read.
    // If input is missing/malformed, behavior in C would be undefined; we unwrap here.
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();

    // Mirror the C chain of comparisons exactly.
    let (d, e, f) = if a <= b && b <= c {
        (a, b, c)
    } else if a <= c && c <= b {
        (a, c, b)
    } else if b <= a && a <= c {
        (b, a, c)
    } else if b <= c && c <= a {
        (b, c, a)
    } else if c <= a && a <= b {
        (c, a, b)
    } else {
        // c <= b && b <= a
        (c, b, a)
    };

    print!("{} {} {}\n", d, e, f);
}
