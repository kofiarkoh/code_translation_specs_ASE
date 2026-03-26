use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the length of the strings
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Read the two strings
    let mut s = String::new();
    let mut t = String::new();
    
    io::stdin().read_line(&mut s).expect("Failed to read string S");
    io::stdin().read_line(&mut t).expect("Failed to read string T");

    // Ensure the strings are of the expected length
    assert_eq!(s.len(), n);
    assert_eq!(t.len(), n);

    let mut ans = 0;
    let mut i = n as isize - 1; // Start from the end of the strings

    while i >= 0 {
        let mut j = n as isize - 1; // Reset j to the end of the string T
        while j >= 0 && s.chars().nth(i as usize) != t.chars().nth(j as usize) {
            j -= 1;
        }
        if j >= 0 {
            let mut k = i; // Use a new variable k to track matches
            while j >= 0 && s.chars().nth(k as usize) == t.chars().nth(j as usize) {
                k -= 1;
                j -= 1;
            }
            if j < 0 {
                ans = 2 * n - (n - (k + 1) as usize);
                break;
            }
        } else {
            ans = 2 * n - (n - (i + 1) as usize);
            break;
        }
        i -= 1; // Move to the next character in S
    }

    // Print the result
    println!("{}", ans);
}
