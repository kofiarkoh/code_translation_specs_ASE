fn main() {
    // Example input: number of circle pairs and their parameters
    let n: usize = 2; // Number of circle pairs
    let circle_pairs = vec![
        (0.0, 0.0, 1.0, 2.0, 0.0, 1.0), // Example circles
        (0.0, 0.0, 1.0, 0.5, 0.0, 0.5), // Example circles
    ];

    for (a_x, a_y, a_r, b_x, b_y, b_r) in circle_pairs {
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
