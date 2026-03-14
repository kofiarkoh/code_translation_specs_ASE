use std::io::{self, BufRead};

const N: usize = 100;

fn hanten(str: &mut [u8], n: usize) {
    let half = n / 2;
    for j in 0..half {
        str.swap(j, n - j - 1);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut cnt = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let mut str1 = line.into_bytes();
        
        // Ensure str1 does not exceed N bytes
        if str1.len() >= N {
            continue; // Skip if input is too long
        }

        let str2 = str1.clone();
        hanten(&mut str1, str1.len());

        // Compare the reversed string with the original
        if str1 == str2 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
