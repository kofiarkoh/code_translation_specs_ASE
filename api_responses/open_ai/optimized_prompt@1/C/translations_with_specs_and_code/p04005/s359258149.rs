use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let values: Vec<long> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    if values.len() != 3 {
        panic!("Expected exactly three long integer values");
    }

    if values[0] % 2 == 0 || values[1] % 2 == 0 || values[2] % 2 == 0 {
        println!("0");
    } else {
        let mut max_index = 0;
        for i in 1..3 {
            if values[i] > values[max_index] {
                max_index = i;
            }
        }

        // Create a mutable copy of the values to modify
        let mut product_values = values.clone();
        product_values[max_index] = 1;

        let product = product_values.iter().product::<long>();
        println!("{}", product);
    }
}
```

### Notes:
1. **Input Handling**: The Rust code reads a single line of input and splits it into three long integers, ensuring that the input is valid.
2. **Even Check**: The check for even numbers is directly translated from the C code.
3. **Finding Maximum**: The maximum value is found using a loop, similar to the C code, but using Rust's indexing.
4. **Product Calculation**: The product is calculated using an iterator over the modified values.
5. **Safety**: The code avoids `unsafe` blocks by using safe Rust constructs, ensuring that all operations are performed within the bounds of the language's safety guarantees.
6. **Output**: The output format is preserved exactly as specified, ensuring that the program behaves as expected.
