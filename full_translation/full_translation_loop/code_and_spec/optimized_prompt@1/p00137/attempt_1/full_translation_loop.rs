use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the number of cases
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let counter: usize = input.trim().parse().expect("Invalid input for number of cases");

    // Ensure the number of cases is positive
    if counter == 0 {
        return; // No cases to process
    }

    for count in 0..counter {
        input.clear();
        
        // Read the integer for calculations
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut x: i32 = input.trim().parse().expect("Invalid input for case value");

        const CTER: i32 = 10;
        println!("Case {}:", count + 1);

        for _ in 0..CTER {
            x = (x * x) / 100; // Square x and divide by 100
            x %= 10000; // Take modulo 10000
            println!("{}", x);
        }
    }
}
