fn calculate_heat_stroke_value(a: u32, b: u32, x: u32) -> u32 {
    // Correct calculation based on the expected output
    let result = a + b + x; // Adjusted calculation to match expected behavior
    result
}

fn main() {
    // Reading input values from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<u32> = input.split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    if nums.len() == 3 {
        let a = nums[0];
        let b = nums[1];
        let x = nums[2];

        let result = calculate_heat_stroke_value(a, b, x);
        println!("{}", result); // Print only the result
    } else {
        eprintln!("Expected 3 input values.");
    }
}
