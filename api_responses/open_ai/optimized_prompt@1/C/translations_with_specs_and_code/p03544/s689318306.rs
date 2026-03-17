use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a non-negative integer");

    // Initialize the first two terms of the sequence
    let mut l = vec![0; n + 1];
    l[0] = 2; // First term
    if n > 0 {
        l[1] = 1; // Second term
    }

    // Compute the nth term using the recurrence relation
    for i in 2..=n {
        l[i] = l[i - 2] + l[i - 1];
    }

    // Output the nth term
    println!("{}", l[n]);
}
```

### Notes:
1. **Input Handling**: The Rust code reads input from standard input and expects a non-negative integer, consistent with the specification.
2. **Vector Initialization**: A vector `l` is used to store the sequence values, initialized to the required size based on the input `n`.
3. **Recurrence Relation**: The loop computes the nth term based on the recurrence relation defined in the specification.
4. **Output**: The output is formatted to match the expected behavior, printing the nth term of the sequence.
5. **Safety**: The code avoids any unsafe operations, adhering to Rust's safety guarantees.
