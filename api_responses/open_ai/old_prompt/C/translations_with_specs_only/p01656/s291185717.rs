fn main() {
    // Read the number of names (N) and the year threshold (Q)
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().unwrap(); // N
    let q: i32 = iter.next().unwrap().parse().unwrap(); // Q

    // Initialize variables to store names and years
    let mut names = Vec::new();
    let mut years = Vec::new();

    // Read N pairs of years and names
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        let year: i32 = parts[0].parse().unwrap(); // year
        let name: &str = parts[1]; // name

        years.push(year);
        names.push(name.to_string());
    }

    // Variable to hold the answer
    let mut answer = &names[0]; // Default to the first name

    // Find the most recent name based on the year threshold Q
    for i in 0..n {
        if years[i] <= q {
            answer = &names[i]; // Update answer if the year is valid
        }
    }

    // Output the result
    println!("{}", answer);
}
