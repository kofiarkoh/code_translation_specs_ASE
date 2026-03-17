use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of cases
    let counter: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input");
    
    // Ensure the number of cases is positive
    if counter == 0 {
        return;
    }

    for count in 0..counter {
        // Read the input value for the case
        let x: i32 = lines.next().unwrap().unwrap().parse().expect("Invalid input");
        
        let mut value = x;
        const ITERATIONS: usize = 10;

        // Output the case header
        println!("Case {}:", count + 1);

        for _ in 0..ITERATIONS {
            // Perform the calculations as specified
            value = (value * value) / 100;
            value %= 10000;

            // Output the computed value
            println!("{}", value);
        }
    }
}
