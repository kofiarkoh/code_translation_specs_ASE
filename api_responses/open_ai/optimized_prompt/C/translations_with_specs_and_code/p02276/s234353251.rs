use std::io::{self, Write};

const N: usize = 1_000_000;
static mut A: [i32; N] = [0; N]; // Using unsafe for global mutable state, as in C

fn partition(p: usize, r: usize) -> usize {
    let x = unsafe { A[r] };
    let mut i = p as isize - 1;

    for j in p..r {
        if unsafe { A[j] } <= x {
            i += 1;
            unsafe {
                A[i as usize] = A[j];
                A[j] = A[i as usize];
            }
        }
    }

    unsafe {
        A[(i + 1) as usize] = A[r];
        A[r] = A[(i + 1) as usize];
    }
    
    (i + 1) as usize
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap(); // Assuming valid input as per spec

    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        unsafe {
            A[i] = input.trim().parse().unwrap(); // Assuming valid input as per spec
        }
    }

    let pivot_index = partition(1, n);
    
    for i in 1..=n {
        if i == pivot_index {
            print!("[{}]", unsafe { A[i] });
        } else {
            print!("{}", unsafe { A[i] });
        }
        if i < n {
            print!(" ");
        }
    }
    println!();
}
```

### Notes:
1. **Global Mutable State**: The use of `static mut` for the array `A` is necessary to mimic the C code's global array behavior. This is marked as `unsafe` because Rust does not allow mutable static variables without unsafe blocks.
  
2. **Input Handling**: The code assumes valid input as per the specification. It does not handle errors for invalid input, consistent with the C code.

3. **Indexing**: The C code uses 1-based indexing, which is preserved in the Rust translation.

4. **Output Formatting**: The output format is preserved exactly as specified, including spaces and brackets around the pivot element.

5. **Safety**: The translation prioritizes safe Rust practices, but the use of `unsafe` is justified due to the need for mutable global state, which is a direct translation from the C code.
