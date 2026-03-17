use std::io;

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse b");

    // Perform the calculations
    let c = a + b;
    let d = a - b;
    let e = a * b;

    // Initialize max to -1000, as in the C code
    let mut max = -1000;

    // Update max if any of the calculated values are greater
    if max < c {
        max = c;
    }
    if max < d {
        max = d;
    }
    if max < e {
        max = e;
    }

    // Print the result
    println!("{}", max);
}
