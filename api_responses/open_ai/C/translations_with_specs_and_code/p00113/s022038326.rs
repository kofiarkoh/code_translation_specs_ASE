use std::io::{self, BufRead};

const MAX: usize = 1_000_000;

fn get_int(input: &mut std::str::Chars) -> Option<usize> {
    let mut n = 0;
    while let Some(c) = input.next() {
        if c.is_ascii_digit() {
            n = n * 10 + (c as usize - '0' as usize);
        } else {
            return Some(n);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut buf = String::new();

    while reader.read_line(&mut buf).unwrap() > 0 {
        let mut chars = buf.chars();
        let a = get_int(&mut chars).expect("Failed to read numerator");
        let b = get_int(&mut chars).expect("Failed to read denominator");

        // Preconditions
        assert!(a <= MAX, "Numerator exceeds maximum limit");
        assert!(b > 0, "Denominator must be positive");

        let mut rr = vec![-1; b];
        let mut sps = vec![' '; 100];
        let mut hat = vec!['^'; 100];

        rr[a] = 0;
        let mut k = 1;
        let mut a = a * 10;

        loop {
            let digit = a / b;
            print!("{}", digit);
            let r = a % b;

            if r == 0 {
                println!();
                break;
            }

            if rr[r] >= 0 {
                println!();
                let i = rr[r];
                let w = k;
                sps[i] = '\0' as char; // Null-terminate for printing
                print!("{}", sps.iter().collect::<String>());
                sps[i] = ' '; // Restore space after printing
                hat[k - w] = '\0' as char; // Null-terminate for printing
                println!("{}", hat.iter().collect::<String>());
                break;
            }

            rr[r] = k;
            a = r * 10; // Prepare for the next iteration
            k += 1;
        }

        buf.clear(); // Clear buffer for the next line
    }
}
```

### Notes:
1. **Input Handling**: The `get_int` function reads integers from a character iterator, similar to the original C function.
2. **Assertions**: Preconditions are enforced using assertions to ensure the numerator is within bounds and the denominator is positive.
3. **Remainders Tracking**: The `rr` vector is used to track remainders, similar to the C code.
4. **Output Formatting**: The output is formatted to match the specification, including the handling of repeating sequences.
5. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees.
6. **Buffer Management**: The buffer is cleared after each line read to prepare for the next input.
