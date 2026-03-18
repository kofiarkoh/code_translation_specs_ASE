use std::io;

fn max(a: i32, b: i32) -> i32 {
    if a >= b { a } else { b }
}

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();

    // Read two strings from stdin
    io::stdin().read_line(&mut s1).expect("Failed to read line");
    io::stdin().read_line(&mut s2).expect("Failed to read line");

    // Trim newline characters
    let s1 = s1.trim();
    let s2 = s2.trim();

    let l_s1 = s1.len();
    let l_s2 = s2.len();

    // Create a 2D vector for dynamic programming table
    let mut arr = vec![vec![0; l_s2 + 1]; l_s1 + 1];

    // Fill the first row and column with 0
    for i in 0..=l_s1 {
        arr[i][0] = 0;
    }
    for j in 0..=l_s2 {
        arr[0][j] = 0;
    }

    // Fill the DP table
    for i in 1..=l_s1 {
        for j in 1..=l_s2 {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                arr[i][j] = arr[i - 1][j - 1] + 1;
            } else {
                arr[i][j] = max(arr[i - 1][j], arr[i][j - 1]);
            }
        }
    }

    let len = arr[l_s1][l_s2];
    let mut ans = vec![' '; len]; // Preallocate space for the answer
    let mut ind = len - 1;
    let mut r = l_s1;
    let mut c = l_s2;

    // Backtrack to find the longest common subsequence
    while r >= 1 && c >= 1 {
        if s1.chars().nth(r - 1) == s2.chars().nth(c - 1) {
            ans[ind] = s1.chars().nth(r - 1).unwrap();
            r -= 1;
            c -= 1;
            ind -= 1;
        } else if arr[r][c] == arr[r - 1][c] {
            r -= 1;
        } else {
            c -= 1;
        }
    }

    // Print the result
    print!("{}", ans.iter().collect::<String>());
}
