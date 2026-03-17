use std::io::{self, BufRead};

fn max(a: usize, b: usize) -> usize {
    if a >= b { a } else { b }
}

fn main() {
    // Read input strings
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    let s1 = input.next().unwrap().unwrap();
    let s2 = input.next().unwrap().unwrap();

    // Ensure the strings do not exceed the maximum length
    assert!(s1.len() <= 2999 && s2.len() <= 2999, "Input strings exceed maximum length");

    let l_s1 = s1.len();
    let l_s2 = s2.len();

    // Create a 2D vector for dynamic programming
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

    // Length of the longest common subsequence
    let len = arr[l_s1][l_s2];
    let mut ans = vec![0; len]; // Allocate space for the LCS

    // Backtrack to find the LCS
    let mut ind = len as isize - 1;
    let mut r = l_s1 as isize;
    let mut c = l_s2 as isize;

    while r >= 1 && c >= 1 {
        if s1.as_bytes()[r as usize - 1] == s2.as_bytes()[c as usize - 1] {
            ans[ind as usize] = s1.as_bytes()[r as usize - 1];
            r -= 1;
            c -= 1;
            ind -= 1;
        } else if arr[r as usize][c as usize] == arr[r as usize - 1][c as usize] {
            r -= 1;
        } else {
            c -= 1;
        }
    }

    // Output the LCS as a string
    let lcs_string = String::from_utf8(ans).unwrap();
    print!("{}", lcs_string);
}
