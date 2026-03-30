use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a non-negative integer");

    // Preconditions check
    if n > 99 {
        panic!("Input exceeds the maximum allowed value of 99.");
    }

    let mut l: [i64; 100] = [0; 100];
    l[0] = 2;
    l[1] = 1;

    for i in 2..=n {
        l[i] = l[i - 2] + l[i - 1];
    }

    println!("{}", l[n]);
}
