use std::io::{self, BufRead};

const N: usize = 1_000_000;
static mut A: [i32; N] = [0; N]; // Using unsafe to allow mutable static array

fn partition(p: usize, r: usize) -> usize {
    unsafe {
        let x = A[r];
        let mut i = p as isize - 1; // Use isize for compatibility with index manipulation
        for j in p..r {
            if A[j] <= x {
                i += 1;
                A.swap(i as usize, j); // Swap elements
            }
        }
        A.swap((i + 1) as usize, r); // Final swap
        (i + 1) as usize // Return the partition index
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for i in 1..=n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        unsafe {
            A[i] = value; // Populate the array
        }
    }

    let x = partition(1, n);
    
    for i in 1..=n {
        if i == x {
            print!("[{}]", unsafe { A[i] });
        } else {
            print!("{}", unsafe { A[i] });
        }
        if i < n {
            print!(" ");
        }
    }
    println!(); // Print newline at the end
}
```

### Explanation of Key Choices:
1. **Static Mutable Array**: The C code uses a global array `A`, which is mutable. In Rust, we use `static mut` to achieve similar behavior, but this requires `unsafe` because mutable static variables can lead to data races in concurrent contexts.
  
2. **Indexing**: The C code uses 1-based indexing for the array, which is preserved in Rust by adjusting the loop indices accordingly.

3. **Swapping Elements**: The `swap` method is used for swapping elements in Rust, which is idiomatic and safe.

4. **Input Handling**: The input is read using `BufRead` for efficiency, and we ensure to trim and parse the input correctly.

5. **Output Formatting**: The output formatting is preserved exactly as in the C code, including spaces and brackets around the pivot element.

6. **Unsafe Code**: The use of `unsafe` is justified due to the need for mutable static state, which is not safe in Rust without explicit handling.
