use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of test cases
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read the time in h:m format
        let time = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = time.split(':').collect();
        let h: i32 = parts[0].parse().unwrap();
        let m: i32 = parts[1].parse().unwrap();

        // Calculate the angle
        let angle = h * 360 / 12 + 360 * m / 12 / 60.0 - 360 * m / 60.0;

        // Adjust angle if greater than 180
        let mut adjusted_angle = if angle > 180.0 {
            angle - 360.0
        } else {
            angle
        };

        // Output based on the angle range
        if adjusted_angle > -30.0 && adjusted_angle < 30.0 {
            println!("alert");
        } else if adjusted_angle > -90.0 && adjusted_angle < 90.0 {
            println!("warning");
        } else {
            println!("safe");
        }
    }
}
