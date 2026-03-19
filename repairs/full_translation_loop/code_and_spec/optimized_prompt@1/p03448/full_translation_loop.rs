use std::io;

fn main() {
    let mut input = String::new();
    
    // Read inputs A, B, C, and X
    io::stdin().read_line(&mut input).unwrap();
    let A: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let B: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let C: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let X: usize = input.trim().parse().unwrap();
    
    let mut count = 0;

    // Iterate through all combinations of i, j, k
    for i in 0..=A {
        for j in 0..=B {
            for k in 0..=C {
                // Check if the weighted sum equals X
                if 500 * i + 100 * j + 50 * k == X {
                    count += 1;
                }
            }
        }
    }

    // Output the count of valid combinations
    println!("{}", count);
}
