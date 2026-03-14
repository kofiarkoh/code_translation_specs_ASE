use std::io::{self, Write};

fn main() {
    let mut a = [0; 5];

    // Read exactly five integers from standard input
    for i in 0..5 {
        // Using `read_line` to read input and then parsing it
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        a[i] = input.trim().parse().expect("Please enter a valid integer");
    }

    // Sort the array in descending order
    a.sort_by(|&x, &y| y.cmp(&x));

    // Print the sorted integers, separated by spaces
    for i in 0..5 {
        if i > 0 {
            print!(" "); // Print space before all but the first number
        }
        print!("{}", a[i]);
    }
    println!(); // Print a newline at the end
}
```

### Notes:
- The Rust code reads five integers from standard input, ensuring that the input is exactly five integers as specified.
- The sorting is done using the `sort_by` method, which sorts the array in descending order by comparing elements.
- The output format is preserved by printing a space between numbers and ensuring a newline at the end.
- Error handling for input parsing is included to ensure that only valid integers are processed, adhering to the specification's preconditions.
