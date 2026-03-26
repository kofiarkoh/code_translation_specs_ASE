use std::io::{self, Write};

fn main() {
    let mut x = [0; 2];
    let mut y = [0; 2];
    let mut r = [0; 2];

    // Read input for two circles
    for i in 0..2 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Please enter valid integers"))
            .collect();
        x[i] = nums[0];
        y[i] = nums[1];
        r[i] = nums[2];
    }

    // Calculate squared distances and radii
    let rdistance = (x[0] - x[1]).pow(2) + (y[0] - y[1]).pow(2);
    let rplus = (r[0] + r[1]).pow(2);
    let rminus = (r[0] - r[1]).pow(2);

    // Determine the relationship between the circles
    let result = if rdistance > rplus {
        4 // Circles are completely separate
    } else if rdistance == rplus {
        3 // Circles are touching externally
    } else if rminus < rdistance && rdistance < rplus {
        2 // Circles are overlapping
    } else if rdistance == rminus {
        1 // Circles are touching internally
    } else {
        0 // Circles are completely separate
    };

    // Print the result
    println!("{}", result);
}
