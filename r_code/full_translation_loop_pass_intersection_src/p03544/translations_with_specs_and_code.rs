use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse one integer n.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.split_whitespace().next().unwrap().parse().unwrap();

    // Fixed-size storage for 100 terms, as in the C code/spec.
    let mut l = [0i64; 100];

    l[0] = 2;
    l[1] = 1;
    for i in 2..=n {
        l[i] = l[i - 2] + l[i - 1];
    }

    print!("{}\n", l[n]);
}
