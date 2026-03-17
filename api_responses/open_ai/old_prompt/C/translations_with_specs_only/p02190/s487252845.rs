fn main() {
    use std::io::{self, BufRead};
    use std::collections::HashSet;

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the first line to get the number of integers
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Handle the case where n is 0
    if n == 0 {
        println!("0");
        return;
    }

    // Read the integers into a vector
    let mut a: Vec<i32> = lines
        .take(n)
        .filter_map(|line| line.ok()?.trim().parse().ok())
        .collect();

    // Sort the array
    a.sort_unstable(); // Using sort_unstable for better performance, as we don't need to maintain order of equal elements

    // Use a HashSet to count unique integers
    let unique_count = a.iter().collect::<HashSet<_>>().len();

    // Print the count of unique integers
    println!("{}", unique_count);
}
