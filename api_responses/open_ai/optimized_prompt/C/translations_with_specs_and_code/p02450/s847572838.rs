fn enumeration(result: &mut Vec<i32>, used: &mut Vec<i32>, n: usize, d: usize) {
    if d >= n {
        // Print the current permutation
        let output = result.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", output);
        return;
    }

    let mut used2 = Vec::with_capacity(n - d - 1);
    for i in 0..(n - d) {
        result[d] = used[i];
        // Create a new used2 vector excluding the current used[i]
        used2.clear();
        for j in 0..(n - d) {
            if j != i {
                used2.push(used[j]);
            }
        }
        enumeration(result, &mut used2, n, d + 1);
    }
}

fn main() {
    let mut n_input = String::new();
    std::io::stdin().read_line(&mut n_input).expect("Failed to read line");
    let n: usize = n_input.trim().parse().expect("Please enter a valid number");

    // Preconditions: n must be a non-negative integer
    if n == 0 {
        return; // No permutations to print for n = 0
    }

    let mut used: Vec<i32> = (1..=n as i32).collect();
    let mut result: Vec<i32> = vec![0; n];

    enumeration(&mut result, &mut used, n, 0);
}
```

### Notes:
1. **Input Handling**: The Rust code reads input using `std::io::stdin()` and trims whitespace, similar to the C code's `scanf`.
2. **Vector Usage**: Rust's `Vec` is used to dynamically manage arrays, which is idiomatic and safe compared to C's static arrays.
3. **Output Formatting**: The output is formatted to match the specification, ensuring space-separated integers and a newline after each permutation.
4. **Error Handling**: The code checks for `n == 0` to avoid unnecessary computation, as there are no permutations to generate.
5. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees.
