use std::io;

fn main() {
    let mut input = String::new();
    
    // Read inputs A, B, C, X
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();
    let c: i32 = input.trim().parse().unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();
    
    let mut count = 0;

    // Iterate over the ranges as in the C code
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                // Check if the combination equals X
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }

    // Print the result
    println!("{}", count);
}
