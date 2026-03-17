fn main() {
    use std::io::{self, Write};

    // Read the number of cases (counter)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let counter: i32 = input.trim().parse().expect("Invalid input for counter");

    // Ensure counter is positive
    if counter <= 0 {
        return; // No output for non-positive counter
    }

    for case_number in 1..=counter {
        // Read the integer input for each case (x)
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut x: i32 = input.trim().parse().expect("Invalid input for x");

        // Print the case number
        println!("Case {}:", case_number);

        // Perform the calculations, repeating 10 times
        for _ in 0..10 {
            x = (x * x) / 100 % 10000; // Square, divide by 100, and take modulo 10000
            println!("{}", x); // Output the result
        }
    }
}
