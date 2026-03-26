use std::io;

fn main() -> Result<(), i32> {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut values: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly 5 integers
    if values.len() != 5 {
        return Err(1); // Invalid input
    }

    let (x, y, a, b, c) = (values[0], values[1], values[2], values[3], values[4]);

    // Validate input constraints
    if x < 0 || y < 0 || a < 0 || b < 0 || c < 0 || a > 114514 || b > 114514 || c > 114514 {
        return Err(1); // Invalid input
    }

    // Initialize arrays
    let mut p: Vec<i32> = vec![0; a as usize];
    let mut q: Vec<i32> = vec![0; b as usize];
    let mut r: Vec<i32> = vec![0; c as usize];

    // Read arrays p, q, and r
    for i in 0..a {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        p[i as usize] = input.trim().parse().expect("Invalid integer");
    }

    for i in 0..b {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        q[i as usize] = input.trim().parse().expect("Invalid integer");
    }

    for i in 0..c {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        r[i as usize] = input.trim().parse().expect("Invalid integer");
    }

    // Sort arrays p and q in descending order
    p.sort_unstable_by(|a, b| b.cmp(a));
    q.sort_unstable_by(|a, b| b.cmp(a));

    // Replace elements in p and q based on r
    for &value in &r {
        if let Some(pos) = p.iter().position(|&x| x < value) {
            p[pos] = value; // Replace the first element in p that is less than value
        }
        if let Some(pos) = q.iter().position(|&x| x < value) {
            q[pos] = value; // Replace the first element in q that is less than value
        }
    }

    // Calculate the sum of the retained elements
    let sum: i64 = p.iter().take(x as usize).map(|&v| v as i64).sum::<i64>()
        + q.iter().take(y as usize).map(|&v| v as i64).sum::<i64>();

    // Output the computed sum
    println!("{}", sum);

    Ok(()) // Successful execution
}
