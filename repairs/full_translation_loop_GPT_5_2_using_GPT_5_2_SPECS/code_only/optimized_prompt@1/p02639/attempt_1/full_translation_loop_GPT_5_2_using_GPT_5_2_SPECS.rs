use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace-separated parsing).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // In the C code, if scanf fails to read all 5 ints, behavior is undefined
    // because a..e would be uninitialized. Here we assume valid input.
    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();
    let d: i32 = it.next().unwrap().parse().unwrap();
    let e: i32 = it.next().unwrap().parse().unwrap();

    if a == 0 {
        print!("1");
    } else if b == 0 {
        print!("2");
    } else if c == 0 {
        print!("3");
    } else if d == 0 {
        print!("4");
    } else if e == 0 {
        print!("5");
    }
}
