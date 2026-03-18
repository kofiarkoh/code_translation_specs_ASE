use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the length of the strings
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a non-negative integer");

    // Allocate space for the strings
    let mut s = String::new();
    let mut t = String::new();

    // Read the first string
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    s.push_str(input.trim());

    // Read the second string
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    t.push_str(input.trim());

    // Ensure the strings are of the expected length
    assert_eq!(s.len(), n);
    assert_eq!(t.len(), n);

    let mut ans = 0;
    let mut i = n.saturating_sub(1); // Start from the last index

    while i < n {
        let mut j = n.saturating_sub(1); // Reset j to the last index for each iteration of i
        while j < n && s.as_bytes()[i] != t.as_bytes()[j] {
            if j == 0 { break; } // Prevent underflow
            j = j.saturating_sub(1); // Decrement j safely
        }
        if j < n {
            while j < n && s.as_bytes()[i] == t.as_bytes()[j] {
                if i == 0 { break; } // Prevent underflow
                i = i.saturating_sub(1);
                j = j.saturating_sub(1);
            }
            if j == usize::MAX {
                ans = 2 * n - (n - i - 1);
                break;
            }
        } else {
            ans = 2 * n - (n - i - 1);
            break;
        }
        if i == 0 { break; } // Prevent underflow
        i = i.saturating_sub(1); // Decrement i safely
    }

    // Print the result
    println!("{}", ans);
}
``` 

### Changes Made:
1. **Loop Control**: Added checks to prevent underflow when decrementing `i` and `j`. This ensures that the loops terminate correctly without running into negative indices.
2. **Resetting `j`**: Changed the initialization of `j` to start from the last index of `t` for each iteration of `i`, which aligns with the original logic.
3. **Break Conditions**: Added break conditions to exit loops when indices reach zero, preventing unnecessary iterations and potential infinite loops.
