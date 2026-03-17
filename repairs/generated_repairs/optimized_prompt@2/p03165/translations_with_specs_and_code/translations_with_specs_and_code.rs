use std::io::{self, BufRead};

fn max(a: usize, b: usize) -> usize {
    if a >= b { a } else { b }
}

fn main() {
    // Read input strings
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    let s1 = input.next().and_then(Result::ok).unwrap_or_default();
    let s2 = input.next().and_then(Result::ok).unwrap_or_default();

    // Preconditions: Check lengths
    let l_s1 = s1.len();
    let l_s2 = s2.len();
    assert!(l_s1 <= 2999 && l_s2 <= 2999, "Input strings exceed maximum length");

    // Create the DP table
    let mut arr = vec![vec![0; l_s2 + 1]; l_s1 + 1];

    // Fill the DP table
    for i in 1..=l_s1 {
        for j in 1..=l_s2 {
            if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                arr[i][j] = arr[i - 1][j - 1] + 1;
            } else {
                arr[i][j] = max(arr[i - 1][j], arr[i][j - 1]);
            }
        }
    }

    // Length of the LCS
    let len = arr[l_s1][l_s2];
    let mut ans = vec!['\0'; len]; // Allocate space for the LCS with char type
    let mut ind = len - 1;
    let mut r = l_s1;
    let mut c = l_s2;

    // Backtrack to find the LCS
    while r > 0 && c > 0 {
        if s1.as_bytes()[r - 1] == s2.as_bytes()[c - 1] {
            ans[ind] = s1.as_bytes()[r - 1] as char; // Store the character
            r -= 1;
            c -= 1;
            ind -= 1;
        } else if arr[r][c] == arr[r - 1][c] {
            r -= 1;
        } else {
            c -= 1;
        }
    }

    // Output the LCS
    for &ch in &ans {
        if ch != '\0' {
            print!("{}", ch);
        }
    }
}
