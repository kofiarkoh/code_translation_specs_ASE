use std::io::{self, Write};

fn main() {
    let mut p = [0; 8];
    let mut c = [0; 16];
    let mut cur_s: i32;
    let mut min_s: i32;
    let mut min_p: usize;

    // Initialize the c array based on the C code
    for i in 0..16 {
        if i % 2 == 0 {
            c[i] = 1;
        }
    }

    c[1] = 2;
    c[3] = 2;
    c[5] = 4;
    c[7] = 4;
    c[9] = 2;
    c[11] = 2;
    c[13] = 4;
    c[15] = 4;

    // Read input until we can't read 8 integers
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let nums: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() != 8 {
            break; // Exit if we don't have exactly 8 integers
        }

        p.copy_from_slice(&nums);

        // Calculate curS based on the initial c array
        cur_s = 0;
        for j in 0..8 {
            if p[j] - c[j] > 0 {
                cur_s += p[j] - c[j];
            }
        }

        min_s = cur_s;

        let mut i = 1; // Start with i = 1
        loop {
            cur_s = 0;

            for j in 0..8 {
                if p[j] - c[i + j] > 0 {
                    cur_s += p[j] - c[i + j];
                }
            }

            if min_s > cur_s {
                min_s = cur_s;
                min_p = i;
            }

            // Update i based on the C code logic
            i = match i {
                1 => 7,
                7 => 3,
                3 => 5,
                5 => 8,
                8 => 6,
                6 => 2,
                2 => 4,
                4 => break, // Exit the loop
                _ => unreachable!(), // This should never happen
            };
        }

        // Print the result
        for i in min_p..(min_p + 8) {
            print!("{}", c[i]);
            if i != (min_p + 7) {
                print!(" ");
            }
        }
        println!(); // Print newline at the end
    }
}
