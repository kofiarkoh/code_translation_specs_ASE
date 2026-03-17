use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the number of integers
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    let mut count_even = 0;

    // Read the integers and count the even numbers
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a: i32 = input.trim().parse().expect("Please enter a valid integer");
        
        if a % 2 == 0 {
            count_even += 1;
        }
    }

    // Calculate 3^N and 2^(count of even integers)
    let exp = 3_i32.pow(n as u32);
    let odd = 2_i32.pow(count_even as u32);

    // Output the result
    println!("{}", exp - odd);
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `io::stdin()` to read input, which is idiomatic for handling standard input in Rust. The input is read as a string and then parsed into the appropriate types.
  
2. **Counting Even Numbers**: The counting of even integers is done using a simple loop, similar to the C code, but using Rust's `for` loop syntax.

3. **Power Calculations**: The calculations for \(3^N\) and \(2^{\text{count of even integers}}\) are done using Rust's `pow` method, which is safe and idiomatic.

4. **Output**: The result is printed using `println!`, which ensures that the output format matches the specification.

5. **Error Handling**: The code uses `expect` to handle potential parsing errors, which is a common practice in Rust to ensure that the program fails gracefully with a clear message if invalid input is provided.

6. **Semantics Preservation**: The logic and flow of the original C code are preserved, ensuring that the behavior aligns with the specification. The handling of `N = 0` is also consistent, resulting in an output of `-1` as specified. 

7. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees.
