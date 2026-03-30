fn calculate_minimum_cost(n: usize, a: i32, b: i32, c: i32, d: i32, x: &[i32], y: &[i32]) -> Result<i32, &'static str> {
    // Check preconditions for input sizes
    if n == 0 || n > 127 {
        return Err("N must be a positive integer and less than or equal to 127.");
    }
    if x.len() != n + 1 || y.len() != n + 1 {
        return Err("Coordinates arrays must have size N+1.");
    }

    // Function to calculate the cost based on boundary constraints
    fn nigeru(x: i32, y: i32, a: i32, b: i32, c: i32, d: i32, is_src: bool) -> i32 {
        let mut cost = 0;

        if x < a {
            cost += a - x;
        } else if x > c {
            cost += x - c;
        }

        if y < b {
            cost += b - y;
        } else if y > d {
            cost += y - d;
        }

        // If it's the source point and cost is greater than zero, decrement by one
        if is_src && cost > 0 {
            cost -= 1;
        }

        cost
    }

    let mut total_cost = 0;

    // Calculate the total movement cost
    for i in 1..=n {
        let direct_distance = ((x[i] - x[i - 1]).abs() + (y[i] - y[i - 1]).abs());
        let cost_from_prev = nigeru(x[i - 1], y[i - 1], a, b, c, d, i == 1);
        let cost_to_next = nigeru(x[i], y[i], a, b, c, d, false);

        // Choose the minimum cost for this segment
        total_cost += direct_distance.min(cost_from_prev + cost_to_next);
    }

    Ok(total_cost)
}

// Example usage (uncomment to test):
// fn main() {
//     let n = 3;
//     let a = 0;
//     let b = 0;
//     let c = 10;
//     let d = 10;
//     let x = [0, 5, 12, 3];
//     let y = [0, 5, 12, 3];
//     match calculate_minimum_cost(n, a, b, c, d, &x, &y) {
//         Ok(cost) => println!("Minimum cost: {}", cost),
//         Err(e) => println!("Error: {}", e),
//     }
// }
