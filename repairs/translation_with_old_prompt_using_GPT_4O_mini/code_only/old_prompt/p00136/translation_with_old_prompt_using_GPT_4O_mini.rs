use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut ca = 0;
    let mut cb = 0;
    let mut cc = 0;
    let mut cd = 0;
    let mut ce = 0;
    let mut cf = 0;

    // Read the number of entries
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let h: f64 = input.trim().parse().expect("Invalid input");

        // Count the height ranges
        if h < 165.0 {
            ca += 1;
        } else if h >= 165.0 && h < 170.0 {
            cb += 1;
        } else if h >= 170.0 && h < 175.0 {
            cc += 1;
        } else if h >= 175.0 && h < 180.0 {
            cd += 1;
        } else if h >= 180.0 && h < 185.0 {
            ce += 1;
        } else if h >= 185.0 {
            cf += 1;
        }
    }

    // Print the results
    print!("1:");
    for _ in 0..ca {
        print!("*");
    }
    println!();

    print!("2:");
    for _ in 0..cb {
        print!("*");
    }
    println!();

    print!("3:");
    for _ in 0..cc {
        print!("*");
    }
    println!();

    print!("4:");
    for _ in 0..cd {
        print!("*");
    }
    println!();

    print!("5:");
    for _ in 0..ce {
        print!("*");
    }
    println!();

    print!("6:");
    for _ in 0..cf {
        print!("*");
    }
    println!();
}
