fn main() {
    // Allocate memory for the input strings
    let mut s1 = String::new();
    let mut s2 = String::new();

    // Read input strings from the user
    std::io::stdin().read_line(&mut s1).expect("Failed to read input");
    std::io::stdin().read_line(&mut s2).expect("Failed to read input");

    // Remove the newline character from the end of the strings
    let s1 = s1.trim();
    let s2 = s2.trim();

    // Ensure the strings do not exceed the maximum length of 2999 characters
    if s1.len() > 2999 || s2.len() > 2999 {
        panic!("Input strings must not exceed 2999 characters.");
    }

    // Calculate the lengths of the input strings
    let n = s1.len();
    let m = s2.len();

    // Create a 2D vector for dynamic programming
    let mut arr = vec![vec![0; m + 1]; n + 1];

    // Populate the dynamic programming array
    for i in 1..=n {
        for j in 1..=m {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                arr[i][j] = arr[i - 1][j - 1] + 1;
            } else {
                arr[i][j] = arr[i - 1][j].max(arr[i][j - 1]);
            }
        }
    }

    // Reconstruct the longest common subsequence from the dynamic programming array
    let mut lcs = String::new();
    let mut i = n;
    let mut j = m;

    while i > 0 && j > 0 {
        if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
            lcs.push(s1.chars().nth(i - 1).unwrap());
            i -= 1;
            j -= 1;
        } else if arr[i - 1][j] >= arr[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    // The LCS is constructed in reverse order, so we need to reverse it
    lcs = lcs.chars().rev().collect();

    // Output the longest common subsequence
    println!("{}", lcs);
}
