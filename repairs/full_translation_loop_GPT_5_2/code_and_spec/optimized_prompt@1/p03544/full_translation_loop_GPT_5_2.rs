use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse first integer as n.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Spec: sequence with first term 2, second term 1, each term is sum of previous two.
    // C code uses an array l[100] and computes up to index n (inclusive).
    // Behavior for negative n is undefined (no handling), so we mirror that assumption.
    let n_usize = n as usize;

    let mut l = vec![0i64; 100];
    l[0] = 2;
    l[1] = 1;
    for i in 2..=n_usize {
        l[i] = l[i - 2] + l[i - 1];
    }

    print!("{}\n", l[n_usize]);
}
