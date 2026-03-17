use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let (a, b, c, d) = (nums[0], nums[1], nums[2], nums[3]);

    // Ensure the input constraints are met
    assert!(0 <= a && a <= 100);
    assert!(0 <= b && b <= 100);
    assert!(0 <= c && c <= 100);
    assert!(0 <= d && d <= 100);
    assert!(a <= b);
    assert!(c <= d);

    let mut overlap_count = 0;

    // Count overlaps
    for i in 0..=100 {
        if (i >= a && i <= b) && (i >= c && i <= d) {
            overlap_count += 1;
        }
    }

    // Output the result according to the specification
    if overlap_count == 0 {
        println!("{}", overlap_count);
    } else {
        println!("{}", overlap_count - 1);
    }
}
