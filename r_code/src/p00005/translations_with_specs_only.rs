use std::io::{self, BufRead};

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                let nums: Vec<&str> = input.split_whitespace().collect();
                if nums.len() == 2 {
                    if let (Ok(a), Ok(b)) = (nums[0].parse::<u64>(), nums[1].parse::<u64>()) {
                        let gcd_value = gcd(a, b);
                        let product_divided_by_gcd = (a * b) / gcd_value;
                        println!("{} {}", gcd_value, product_divided_by_gcd);
                    }
                    // If parsing fails, the behavior is not defined as per the specification.
                }
            }
            // If reading the line fails, we do not handle it as per the specification.
        }
    }
}
```

### Explanation of Choices:
1. **GCD Function**: The `gcd` function implements the Euclidean algorithm recursively, which is a standard method for computing the GCD.
2. **Input Handling**: The program reads from standard input until EOF, processing each line independently. It splits the line into two parts and attempts to parse them as `u64` to ensure non-negativity.
3. **Error Handling**: As per the specification, if the input format is incorrect or non-integer values are provided, the program does not handle these cases explicitly. Thus, we simply do not output anything in such cases.
4. **Output**: The program prints the GCD and the product divided by the GCD, ensuring both values are non-negative integers as guaranteed by the preconditions and postconditions.
5. **Use of `u64`**: This type is chosen to avoid negative values and to handle larger integers, as the specification does not limit the size of the input integers.
