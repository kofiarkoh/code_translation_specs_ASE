use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens (matches scanf("%d%s") behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Per spec: undefined behavior on malformed/missing input; we unwrap accordingly.
    let n: usize = it.next().unwrap().parse().unwrap();
    let q: i32 = it.next().unwrap().parse().unwrap();

    // Default name as in the C code initialization.
    let mut answer = String::from("kogakubu10gokan");

    // Process N records in input order; update answer when year <= query year.
    for _ in 0..n {
        let year: i32 = it.next().unwrap().parse().unwrap();
        let name = it.next().unwrap(); // token (non-whitespace)
        if year <= q {
            answer.clear();
            answer.push_str(name);
        }
    }

    // Output exactly one name token followed by a newline.
    println!("{answer}");
}
