// Aizu Vol-1 0113: Period
// 2017.10.23 bal4u@uu

use std::io::{self, BufRead};

const MAX: usize = 1_000_000;
const BUF_SIZE: usize = 130;

fn get_int(p: &mut &str) -> i32 {
    let mut n = 0;
    if p.starts_with(' ') {
        *p = &p[1..]; // Skip the space
    }
    while let Some(c) = p.chars().next() {
        if c < '0' {
            break;
        }
        n = (n << 3) + (n << 1) + (c as i32 - '0' as i32);
        *p = &p[1..]; // Move to the next character
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut rr = vec![-1; MAX + 2];
    let mut sps = vec![' '; 100];
    let mut hat = vec!['^'; 100];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut p = line.as_str();
        let a = get_int(&mut p);
        let b = get_int(&mut p);
        
        rr.fill(-1); // Reset rr array
        rr[a as usize] = 0;
        let mut k = 1;
        let mut a = a * 10;

        loop {
            print!("{}", a / b);
            let r = a % b;
            if r == 0 {
                println!();
                break;
            }
            if rr[r as usize] >= 0 {
                println!();
                let i = rr[r as usize];
                let w = i;
                sps[i as usize] = '\0'; // Null-terminate for printing
                print!("{}", sps.iter().collect::<String>());
                sps[i as usize] = ' '; // Restore space
                hat[(k - w) as usize] = '\0'; // Null-terminate for printing
                println!("{}", hat.iter().collect::<String>());
                hat[(k - w) as usize] = '^'; // Restore caret
                break;
            }
            rr[r as usize] = k;
            a = r * 10; // Update a for the next iteration
            k += 1;
        }
    }
}
```

### Explanation of Key Choices:
1. **Input Handling**: Used `std::io` for reading input, which is idiomatic in Rust.
2. **Array Initialization**: Used `vec!` for dynamic array allocation and `fill` method to reset the `rr` array.
3. **Character Manipulation**: Handled character manipulation using Rust's string slices and character methods.
4. **Loop and Conditionals**: Preserved the logic of the C code, including the infinite loop and break conditions.
5. **Output Formatting**: Used `print!` and `println!` to match the output behavior of the original C code.
6. **Null-Termination**: Used `'\0'` to mimic the C-style string behavior for `sps` and `hat`, although Rust strings are not null-terminated, this is done for compatibility with the original logic.
