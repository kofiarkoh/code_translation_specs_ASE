use std::io::{self, Read};

fn main() {
    // The judge input may contain multiple lines, but the specification states
    // the program reads a single integer value. Therefore, only the first token
    // is consumed; any remaining tokens are ignored.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let n: i64 = s.split_whitespace().next().unwrap().parse().unwrap();

    // Compute based on integer division by 11 and remainder when divided by 11.
    // Use Euclidean division to keep remainder in 0..=10.
    let q = n.div_euclid(11);
    let r = n.rem_euclid(11);

    // Narrowest interpretation consistent with the spec and sample:
    // output = (n / 11) + (n % 11).
    let out = q + r;

    print!("{out}");
}
