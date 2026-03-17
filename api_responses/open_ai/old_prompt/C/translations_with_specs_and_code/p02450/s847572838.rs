use std::io::{self, BufRead};

fn enumeration(result: &mut Vec<i32>, used: &mut Vec<i32>, n: usize, d: usize) {
    if d >= n {
        // Print the current permutation
        let output: Vec<String> = result.iter().map(|&x| x.to_string()).collect();
        println!("{}", output.join(" "));
        return;
    }

    let mut used2 = Vec::with_capacity(n - d - 1);
    for i in 0..(n - d) {
        result[d] = used[i];
        // Fill used2 with elements from used excluding the current one
        for j in 0..(n - d) {
            if j != i {
                used2.push(used[j]);
            }
        }
        enumeration(result, &mut used2, n, d + 1);
        used2.clear(); // Clear used2 for the next iteration
    }
}

fn main() {
    let stdin = io::stdin();
    let n: usize = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();

    if n == 0 {
        return; // If n is 0, no output is generated
    }

    let mut used: Vec<i32> = (1..=n as i32).collect();
    let mut result: Vec<i32> = vec![0; n];

    enumeration(&mut result, &mut used, n, 0);
}
```

### Notes:
1. **Input Handling**: The Rust code reads from standard input and expects a single integer. It uses `BufRead` for efficient reading.
2. **Dynamic Arrays**: Rust's `Vec` is used instead of fixed-size arrays, allowing for dynamic sizing and memory safety.
3. **Output Formatting**: The output is formatted to match the specification, ensuring integers are separated by spaces and printed on new lines.
4. **Error Handling**: The code does not explicitly handle invalid input (like negative integers) as per the specification, but it does check for `n == 0` to avoid unnecessary output.
5. **Memory Safety**: The use of `Vec` ensures that memory is managed safely without the need for `unsafe` blocks.
