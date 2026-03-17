fn main() {
    // Read input values for n and m
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().unwrap(); // Number of integers in the array
    let m: usize = iter.next().unwrap().parse().unwrap(); // Maximum divisor for modulo operation
    
    // Read the array of integers
    let mut a: Vec<i32> = iter.map(|x| x.parse().unwrap()).collect();
    
    // Ensure the array has exactly n elements
    assert_eq!(a.len(), n, "Array length does not match n");

    // Sort the array based on the modulo operation with divisors from 1 to m
    for k in 1..=m {
        a.sort_by_key(|&x| (x % k + k) % k); // Sort by modulo k, ensuring non-negative results
    }

    // Output the sorted array, each integer on a new line
    for &value in &a {
        println!("{}", value);
    }
}
