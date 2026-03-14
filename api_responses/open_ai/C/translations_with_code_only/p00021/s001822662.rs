use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        // Read the coordinates
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<f64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (x1, y1, x2, y2, x3, y3, x4, y4) = (
            coords[0], coords[1], coords[2], coords[3],
            coords[4], coords[5], coords[6], coords[7]
        );

        // Calculate slopes
        let sl1 = (y2 - y1) / (x2 - x1);
        let sl2 = (y4 - y3) / (x4 - x3);

        // Compare slopes
        if sl1 == sl2 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
