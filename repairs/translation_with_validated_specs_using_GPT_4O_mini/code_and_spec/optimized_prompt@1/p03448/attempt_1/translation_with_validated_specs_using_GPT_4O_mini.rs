use std::io;

fn main() {
    let mut input = String::new();
    
    // Read inputs A, B, C, and X
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    let mut count = 0;

    // Iterate through all combinations of i, j, and k
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                // Check if the weighted sum equals X
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }

    // Print the result
    println!("{}", count);
}
