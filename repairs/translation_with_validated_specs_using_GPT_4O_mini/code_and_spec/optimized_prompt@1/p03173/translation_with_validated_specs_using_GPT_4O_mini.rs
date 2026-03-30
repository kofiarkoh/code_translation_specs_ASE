use std::io::{self, BufRead};

const SUP: i64 = 1 << 60; // Equivalent to (long long)1 << 60 in C

// Global variables
static mut N: usize = 0; // Number of elements
static mut A: [i64; 401] = [0; 401]; // Input array
static mut DP: [[i64; 401]; 401] = [[0; 401]; 401]; // Memoization table
static mut SUM: [i64; 401] = [0; 401]; // Cumulative sum

// Recursive function to compute minimum cost of partitioning
fn recursion(l: usize, r: usize) -> i64 {
    unsafe {
        // If already computed or l == r, return the stored value
        if DP[l][r] != 0 || l == r {
            return DP[l][r];
        } else {
            DP[l][r] = SUP; // Initialize to a large value
        }

        // Iterate through possible partition points
        for k in l + 1..=r {
            let tmp = recursion(l, k - 1) + recursion(k, r);
            if tmp < DP[l][r] {
                DP[l][r] = tmp; // Update minimum cost
            }
        }
        DP[l][r] += SUM[r] - SUM[l - 1]; // Add the cost of the current segment
        return DP[l][r];
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    unsafe {
        if let Some(Ok(line)) = lines.next() {
            N = line.trim().parse().expect("Failed to parse N");
        }
        for i in 1..=N {
            if let Some(Ok(line)) = lines.next() {
                A[i] = line.trim().parse().expect("Failed to parse A[i]");
            }
        }
        // Compute cumulative sums
        SUM[0] = 0; // Initialize SUM[0]
        for i in 1..=N {
            SUM[i] = SUM[i - 1] + A[i];
        }
        // Print the result of the recursion
        println!("{}", recursion(1, N));
    }
}
``` 

### Changes Made:
1. **Error Handling**: Added `expect` to the parsing of `N` and `A[i]` to provide clearer error messages if parsing fails, which should help diagnose issues with input.
2. **Trimming Input**: Used `trim()` to remove any leading or trailing whitespace from the input lines before parsing, which can prevent `ParseIntError` due to unexpected whitespace.
