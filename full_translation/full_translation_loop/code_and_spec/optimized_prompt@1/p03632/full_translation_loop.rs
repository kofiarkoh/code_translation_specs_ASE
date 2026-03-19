use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    let (a, b, c, d) = (nums[0], nums[1], nums[2], nums[3]);

    // Preconditions check
    if !(0 <= a && a <= 100 && 0 <= b && b <= 100 && a <= b && 0 <= c && c <= 100 && 0 <= d && d <= 100 && c <= d) {
        // Behavior is not defined for invalid input as per the specification
        return;
    }

    let mut a_range = [0; 101];
    let mut b_range = [0; 101];
    let mut overlap_count = 0;

    // Mark the first range
    for i in a..=b {
        a_range[i as usize] = 1;
    }

    // Mark the second range
    for i in c..=d {
        b_range[i as usize] = 1;
    }

    // Count overlaps
    for i in 0..=100 {
        if a_range[i] + b_range[i] == 2 {
            overlap_count += 1;
        }
    }

    // Adjust output based on overlap count
    if overlap_count == 0 {
        println!("{}", overlap_count);
    } else {
        println!("{}", overlap_count - 1);
    }
}
