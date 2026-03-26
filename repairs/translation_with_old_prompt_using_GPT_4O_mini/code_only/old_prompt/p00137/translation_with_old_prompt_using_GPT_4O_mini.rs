use std::io::{self, Write};

fn main() {
    let mut count = 0;
    let mut counter = String::new();
    
    // Read the number of cases
    io::stdin().read_line(&mut counter).unwrap();
    let counter: i32 = counter.trim().parse().unwrap();

    while counter > count {
        let mut x = String::new();
        
        // Read the value of x
        io::stdin().read_line(&mut x).unwrap();
        let mut x: i32 = x.trim().parse().unwrap();
        let cter = 10;
        let mut c = 0;

        // Print the case number
        println!("Case {}:", count + 1);
        
        while c < cter {
            // Perform the operations on x
            x = (x * x) / 100; // Square x and divide by 100
            x %= 10000; // Take modulo 10000
            c += 1;

            // Print the result
            println!("{}", x);
        }
        count += 1;
    }
}
