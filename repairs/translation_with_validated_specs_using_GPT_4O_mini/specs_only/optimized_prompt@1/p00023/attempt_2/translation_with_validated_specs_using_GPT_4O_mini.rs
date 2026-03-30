use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of circle pairs
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        // Read the parameters for each pair of circles
        let line = lines.next().unwrap().unwrap();
        let params: Vec<f64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (a_x, a_y, a_r, b_x, b_y, b_r) = (params[0], params[1], params[2], params[3], params[4], params[5]);
        let result = determine_circle_relationship(a_x, a_y, a_r, b_x, b_y, b_r);
        println!("{}", result);
    }
}

// Function to determine the relationship between two circles
fn determine_circle_relationship(a_x: f64, a_y: f64, a_r: f64, b_x: f64, b_y: f64, b_r: f64) -> i32 {
    // Calculate squared distance between the centers
    let dx = a_x - b_x;
    let dy = a_y - b_y;
    let dist_sq = dx * dx + dy * dy;

    // Calculate squared radii
    let r_sum_sq = (a_r + b_r) * (a_r + b_r);
    let r_diff_sq = (a_r - b_r).abs() * (a_r - b_r).abs();

    // Determine the relationship based on the distances and radii
    if dist_sq > r_sum_sq {
        // Circles do not intersect
        return 0;
    } else if dist_sq == r_sum_sq {
        // Circles intersect at one point (externally tangent)
        return 1;
    } else if dist_sq < r_diff_sq {
        // One circle is completely within the other
        if a_r < b_r {
            return 2; // Circle a is within circle b
        } else {
            return -2; // Circle b is within circle a
        }
    } else {
        // Circles intersect at two points
        return 1;
    }
}
