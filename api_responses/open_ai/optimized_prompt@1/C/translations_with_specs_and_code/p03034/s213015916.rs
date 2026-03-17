fn main() {
    use std::io::{self, BufRead};

    // Read input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");
    
    // Preconditions check
    if n < 3 {
        panic!("Input must have at least 3 elements");
    }

    // Read the array elements
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for line in lines.take(n) {
        let value: i64 = line.unwrap().trim().parse().expect("Invalid input");
        a.push(value);
    }

    let mut ans = 0;

    // Iterate over possible distances
    for d in 1..(n - 1) / 2 {
        let mut s = 0;
        let mut g = n - 1;
        let mut crr = 0;

        loop {
            let ns = s + d;
            let ng = g - d;

            // Check the exit conditions
            if ng <= d || ns == ng || ns == g {
                break;
            }

            // Update indices and cumulative sum
            s = ns;
            g = ng;
            crr += a[s] + a[g];
            ans = ans.max(crr);
        }
    }

    // Output the result
    println!("{}", ans);
}
```

### Notes:
1. **Preconditions Handling**: The Rust code checks if `n < 3` and panics if this condition is not met, which aligns with the specification's requirement for preconditions.
2. **Array Handling**: The array is read into a `Vec<i64>`, which is dynamically sized and safe in Rust.
3. **Loop Logic**: The loop structure and logic are preserved from the C code, ensuring that the distance constraints are maintained.
4. **Output**: The output format is preserved with `println!`, ensuring that the result is printed as specified.
5. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees.
