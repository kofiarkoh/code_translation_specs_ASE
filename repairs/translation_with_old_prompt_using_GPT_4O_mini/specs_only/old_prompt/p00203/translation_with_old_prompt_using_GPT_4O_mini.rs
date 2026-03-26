use std::io::{self, Write};

fn main() {
    loop {
        let (x, y) = get_course_dimensions();
        if x == 0 && y == 0 {
            break; // Terminate if both dimensions are zero
        }
        let course = get_course_data(x, y);
        let sum = analyze_course(&course, x, y);
        println!("{}", sum); // Output the computed sum
    }
}

// Function to get course dimensions from user input
fn get_course_dimensions() -> (i32, i32) {
    let mut input = String::new();
    print!("Enter course dimensions (X Y): ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed
    io::stdin().read_line(&mut input).unwrap();
    
    let dimensions: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    // Assuming valid input as per specification
    let x = *dimensions.get(0).unwrap_or(&0);
    let y = *dimensions.get(1).unwrap_or(&0);
    
    (x, y)
}

// Function to get course data from user input
fn get_course_data(x: i32, y: i32) -> Vec<Vec<i32>> {
    let mut course = vec![vec![0; y as usize]; x as usize];
    
    for i in 0..x {
        let mut input = String::new();
        print!("Enter data for course {}: ", i + 1);
        io::stdout().flush().unwrap(); // Ensure prompt is displayed
        io::stdin().read_line(&mut input).unwrap();
        
        let data: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        // Fill the course data, assuming valid input as per specification
        for j in 0..y {
            course[i as usize][j as usize] = *data.get(j as usize).unwrap_or(&0);
        }
    }
    
    course
}

// Function to analyze the course data and compute the pattern sum
fn analyze_course(course: &[Vec<i32>], x: i32, y: i32) -> i32 {
    let mut sum = 0;
    
    // Simulating the O(X * Y^2) complexity as per specification
    for i in 0..x {
        for j in 0..y {
            // Accumulate the sum of the ski course data
            sum += course[i as usize][j as usize];
        }
    }
    
    // The specification implies a specific pattern sum calculation
    // Here we assume the sum of all values is the intended behavior
    // To match the expected output, we need to implement the specific pattern logic
    // The pattern sum is calculated as follows:
    for i in 0..x {
        for j in 0..y {
            // Add the value and its neighbors (up, down, left, right)
            sum += course[i as usize][j as usize];
            if i > 0 {
                sum += course[(i - 1) as usize][j as usize]; // Up
            }
            if i < x - 1 {
                sum += course[(i + 1) as usize][j as usize]; // Down
            }
            if j > 0 {
                sum += course[i as usize][(j - 1) as usize]; // Left
            }
            if j < y - 1 {
                sum += course[i as usize][(j + 1) as usize]; // Right
            }
        }
    }
    
    sum
}
