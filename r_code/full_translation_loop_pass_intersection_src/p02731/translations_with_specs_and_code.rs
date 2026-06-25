use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let l: i32 = input.split_whitespace().next().unwrap().parse().unwrap();

    let y: f64 = (l as f64) / 3.0;
    let x: f64 = y * y * y;

    // Match C's printf("%.12lf\n", x);
    println!("{:.12}", x);
}
