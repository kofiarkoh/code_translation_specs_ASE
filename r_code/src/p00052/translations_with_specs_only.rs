use std::io::{self, BufRead};

fn count_factors_of_5(n: u64) -> u64 {
    let mut count = 0;
    let mut power_of_5 = 5;

    // Count the number of factors of 5 in n!
    while n >= power_of_5 {
        count += n / power_of_5;
        power_of_5 *= 5;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.expect("Failed to read line");
        let number: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };

        if number == 0 {
            break; // Termination condition
        }

        let result = count_factors_of_5(number);
        println!("{}", result);
    }
}
```

### Explanation of Choices:
1. **Functionality**: The `count_factors_of_5` function calculates the number of factors of 5 in the factorial of a given non-negative integer. This is done using the method of counting multiples of powers of 5.
2. **Input Handling**: The program reads from standard input until a zero is encountered, which serves as the termination condition.
3. **Output**: Each result is printed on a new line, adhering to the specification.
4. **Error Handling**: Invalid inputs are ignored, as the specification does not define behavior for them.
5. **Data Types**: `u64` is used to handle non-negative integers, ensuring that we can work with larger values without overflow in typical use cases.
