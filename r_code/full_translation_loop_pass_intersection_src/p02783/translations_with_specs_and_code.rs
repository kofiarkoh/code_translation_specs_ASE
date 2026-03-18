use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    // Spec: if b == 0, behavior is undefined. We do not add checks.
    // Preserve C semantics: integer division truncates toward zero; remainder has sign of `a`.
    let mut c = a / b;
    if a % b != 0 {
        c += 1;
    }

    print!("{c}\n");
}
