// AOJ 341 Repeated Spell
// 2017.10.26

use std::io::{self, Read};

const M: i64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t = lines.next().unwrap();
    let b = lines.next().unwrap();
    
    let st = t.len();
    let sb = b.len();
    
    // Create a 2D vector initialized to 0
    let mut a = vec![vec![0; sb + 1]; st + 1];
    
    // Base case: a[i][0] = 1 for all i
    for i in 0..=st {
        a[i][0] = 1;
    }
    
    // Fill the DP table
    for i in 1..=st {
        for j in 0..sb {
            a[i][j + 1] = a[i - 1][j + 1] % M; // Modulo operation to prevent overflow
            if t.as_bytes()[i - 1] == b.as_bytes()[j] {
                a[i][j + 1] = (a[i][j + 1] + a[i - 1][j]) % M; // Add the previous value if characters match
            }
        }
    }
    
    // Output the result
    println!("{}", a[st][sb]);
}
