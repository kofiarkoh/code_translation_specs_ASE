fn main() {
    // Read the number of integers
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for n");

    // Check precondition: n must be at least 3 to form triplets
    if n < 3 {
        println!("0");
        return;
    }

    // Read the integers into a vector
    let mut nums = Vec::with_capacity(n);
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    for num_str in input.trim().split_whitespace() {
        let num: i32 = num_str.parse().expect("Invalid integer input");
        nums.push(num);
    }

    // Invariant: nums should contain exactly n integers
    assert_eq!(nums.len(), n);

    let mut count = 0; // Initialize count of valid triplets

    // Iterate through the array to count valid triplets
    for i in 0..(n - 2) {
        if (nums[i] < nums[i + 1] && nums[i + 1] < nums[i + 2]) || // strictly increasing
           (nums[i] > nums[i + 1] && nums[i + 1] > nums[i + 2]) { // strictly decreasing
            count += 1; // Increment count for valid triplet
        }
    }

    // Output the count of valid triplets
    println!("{}", count);
}
