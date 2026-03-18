use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the first integer N
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    
    input.clear();
    
    // Read the second and third integers A and B
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    
    let mut count = 0;

    for i in 1..=n {
        let e = i % 10;
        let d = (i / 10) % 10;
        let c = (i / 100) % 10;
        let b_digit = (i / 1000) % 10; // Renamed to avoid shadowing
        let a_digit = (i / 10000) % 10; // Renamed to avoid shadowing
        
        let sum = a_digit + b_digit + c + d + e;
        
        // Check if sum is within the range [A, B]
        if a <= sum && sum <= b {
            count += i;
        }
    }
    
    // Print the result
    println!("{}", count);
}
``` 

### Changes Made:
1. **Variable Shadowing**: The variables `a` and `b` were shadowed in the loop. I renamed them to `a_digit` and `b_digit` to avoid confusion and potential issues.
2. **Error Handling**: The `unwrap()` calls can panic if the input is not as expected. In a production scenario, you would want to handle these errors gracefully, but for this translation, I kept them as is to maintain the original behavior. 

Please ensure that the input format matches the expected format in the C code to avoid runtime errors.
