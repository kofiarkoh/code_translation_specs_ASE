fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line to get `n`
    let first_line = lines.next().unwrap().unwrap();
    let mut n: i32 = first_line.trim().parse().unwrap();
    
    // Initialize the array with a size of 1 << 18
    let mut m = [0; 1 << 18];
    let mut p: *mut i32 = m.as_mut_ptr(); // Pointer to the array

    // Read subsequent lines until EOF
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        if nums.len() < 2 {
            break; // If there are not enough numbers, exit the loop
        }
        
        let a = nums[0];
        let b = nums[1];

        // Emulate the behavior of the original C code
        if !p.is_null() && n != 0 {
            n = 0;
        } else {
            unsafe {
                *p += 1; // Increment the value pointed to by `p`
            }
        }

        // Update `p` based on the values of `a` and `b`
        if a != 1 {
            if b != n {
                p = m.as_mut_ptr(); // Reset pointer to the start of the array
            } else {
                unsafe {
                    p = m.as_mut_ptr().add(a as usize - 1); // Move pointer
                }
            }
        } else {
            unsafe {
                p = m.as_mut_ptr().add(b as usize); // Move pointer
            }
        }
    }

    // Output the result based on the value of `n`
    if n != 0 {
        println!("IMPOSSIBLE");
    } else {
        println!("POSSIBLE");
    }
}
