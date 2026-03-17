use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read three long integers from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<long> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Check if any value is even
    if values.iter().any(|&x| x % 2 == 0) {
        println!("0");
    } else {
        // All values are odd, find the maximum
        let mut max_index = 0;
        for i in 1..3 {
            if values[i] > values[max_index] {
                max_index = i;
            }
        }

        // Create a mutable copy of the values to modify the maximum
        let mut product = values.clone();
        product[max_index] = 1; // Set the maximum value to 1

        // Calculate the product of the three values
        let result = product.iter().product::<long>();
        println!("{}", result);
    }
}
```

### Notes:
1. **Input Handling**: The Rust code reads a single line of input and splits it into three long integers, similar to the C code's `scanf`.
2. **Even Check**: The check for even numbers is done using `iter().any()`, which is idiomatic in Rust.
3. **Finding Maximum**: The maximum value is found using a simple loop, similar to the C code, but we start from index 1 since we initialize `max_index` to 0.
4. **Product Calculation**: The product is calculated using `iter().product()`, which is a safe and idiomatic way to compute the product of elements in a collection.
5. **Output**: The output format is preserved exactly as specified, ensuring that the program behaves as expected in terms of I/O.
6. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees.
