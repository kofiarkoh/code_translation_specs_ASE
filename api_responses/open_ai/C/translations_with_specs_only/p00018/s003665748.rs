fn main() {
    // Create a vector to hold exactly five integers
    let mut numbers = Vec::new();

    // Read five integers from standard input
    for _ in 0..5 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse the input string into an integer
        let number: i32 = input.trim().parse().expect("Please enter a valid integer");
        numbers.push(number);
    }

    // Sort the numbers in descending order
    numbers.sort_by(|a, b| b.cmp(a));

    // Print the sorted numbers, separated by spaces
    println!("{}", numbers.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" "));
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads exactly five integers, as specified. If fewer or more integers are provided, the behavior is undefined, which is preserved by the loop structure.
2. **Sorting**: The sorting is done in descending order using `sort_by`, which directly reflects the requirement.
3. **Output**: The output format is maintained by joining the integers with spaces.
4. **Error Handling**: The use of `expect` during parsing ensures that if the input is not a valid integer, the program will panic, which aligns with the undefined behavior specified for incorrect input sizes.
5. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees while fulfilling the specification's requirements.
