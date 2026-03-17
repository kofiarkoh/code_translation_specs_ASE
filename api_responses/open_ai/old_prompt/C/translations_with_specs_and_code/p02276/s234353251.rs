use std::io::{self, BufRead};

const N: usize = 1_000_000;
static mut A: [i32; N] = [0; N];

fn partition(p: usize, r: usize) -> usize {
    let x = unsafe { A[r] }; // Accessing the global array A
    let mut i = p as isize - 1; // Convert to isize for indexing

    for j in p..r {
        if unsafe { A[j] } <= x {
            i += 1;
            unsafe {
                A.swap(i as usize, j); // Swap elements safely
            }
        }
    }
    unsafe {
        A.swap((i + 1) as usize, r); // Final swap to place pivot
    }
    (i + 1) as usize // Return the new pivot index
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for i in 1..=n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        unsafe {
            A[i] = value; // Populate the global array A
        }
    }

    let x = partition(1, n);
    for i in 1..=n {
        if i == x {
            print!("[{}]", unsafe { A[i] }); // Print pivot in brackets
        } else {
            print!("{}", unsafe { A[i] });
        }
        if i < n {
            print!(" "); // Print space between elements
        }
    }
    println!(); // Newline at the end
}
```

### Notes:
1. **Global Array**: The array `A` is declared as a static mutable array to mimic the global state in C. Accessing it is done within `unsafe` blocks, as Rust requires explicit handling of mutable static variables.
2. **Indexing**: The indices in Rust are zero-based, while the C code uses one-based indexing. Adjustments are made accordingly.
3. **Swapping**: The `swap` method is used for swapping elements, which is safe and idiomatic in Rust.
4. **Input Handling**: The input is read using `BufRead` for efficiency, especially with potentially large input sizes.
5. **Output Formatting**: The output format is preserved exactly as specified, including spaces and brackets around the pivot.
