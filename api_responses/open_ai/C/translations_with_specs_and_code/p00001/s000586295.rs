use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut rank = [0; 3];
    let mut unique_count = 0;

    for line in handle.lines().take(10) {
        let work: i32 = line.unwrap().trim().parse().expect("Invalid input");
        
        // Update the rank array based on the current value
        if unique_count < 3 {
            if unique_count == 0 || work > rank[0] {
                rank[2] = rank[1];
                rank[1] = rank[0];
                rank[0] = work;
                unique_count += 1;
            } else if work > rank[1] && work != rank[0] {
                rank[2] = rank[1];
                rank[1] = work;
                unique_count += 1;
            } else if work > rank[2] && work != rank[0] && work != rank[1] {
                rank[2] = work;
                unique_count += 1;
            }
        } else {
            // Only update if the new value is unique and greater than the smallest in rank
            if work > rank[2] && work != rank[0] && work != rank[1] {
                rank[2] = work;
            }
        }
    }

    // Output the top three unique values
    for i in 0..3 {
        if i < unique_count {
            println!("{}", rank[i]);
        }
    }
}
