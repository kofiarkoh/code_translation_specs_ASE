use std::io::{self, BufRead};

const MEW: usize = 112345;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read N and W
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let N = nums.next().unwrap();
    let W = nums.next().unwrap();

    let mut w = vec![0; 128];
    let mut v = vec![0; 128];

    // Read weights and values
    for i in 0..N {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        w[i as usize] = nums.next().unwrap();
        v[i as usize] = nums.next().unwrap();
    }

    let mut ayaneru = vec![W + 1; MEW];
    ayaneru[0] = 0;

    // Dynamic programming to fill ayaneru
    for i in 0..N {
        let vi = v[i as usize];
        let wi = w[i as usize];
        for j in (vi..=MEW as i32 - 1).rev() {
            if ayaneru[(j - vi) as usize] + wi < ayaneru[j as usize] {
                ayaneru[j as usize] = ayaneru[(j - vi) as usize] + wi;
            }
        }
    }

    // Find the maximum index with value <= W
    for i in (0..=MEW as i32 - 1).rev() {
        if ayaneru[i as usize] <= W {
            println!("{}", i);
            return;
        }
    }

    // If no valid index found
    println!("error!");
}
