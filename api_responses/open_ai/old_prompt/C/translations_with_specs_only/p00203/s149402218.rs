use std::io::{self, Write};

fn main() {
    loop {
        let (x, y) = get_course_dimensions();
        if x == 0 && y == 0 {
            println!("0");
            break;
        }
        let course = get_course_data(x, y);
        let sum = analyze_course(&course, x, y);
        println!("{}", sum);
    }
}

// Function to get course dimensions from user input
fn get_course_dimensions() -> (usize, usize) {
    let mut input = String::new();
    print!("Enter number of courses (X) and data points per course (Y): ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let dimensions: Vec<usize> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Assuming valid input as per specification
    let x = if dimensions.len() > 0 { dimensions[0] } else { 0 };
    let y = if dimensions.len() > 1 { dimensions[1] } else { 0 };
    
    (x, y)
}

// Function to get course data from user input
fn get_course_data(x: usize, y: usize) -> Vec<Vec<i32>> {
    let mut course = vec![vec![0; y]; x];
    for i in 0..x {
        let mut input = String::new();
        print!("Enter data for course {}: ", i + 1);
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let data: Vec<i32> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Fill the course data, assuming valid input as per specification
        for j in 0..y {
            if j < data.len() {
                course[i][j] = data[j];
            }
        }
    }
    course
}

// Function to analyze the course data and compute the pattern sum
fn analyze_course(course: &[Vec<i32>], x: usize, y: usize) -> i32 {
    let mut sum = 0;
    for i in 0..x {
        for j in 0..y {
            // Example pattern analysis: sum all values
            sum += course[i][j];
        }
    }
    sum
}
