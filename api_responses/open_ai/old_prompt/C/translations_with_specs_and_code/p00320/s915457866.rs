use std::io::{self, BufRead};

fn next_permutation(arr: &mut [usize]) -> bool {
    let n = arr.len();
    let mut i = n - 2;

    while i >= 0 && arr[i] >= arr[i + 1] {
        if i == 0 { return false; }
        i -= 1;
    }
    let target = i;

    let mut j = n - 1;
    while arr[j] <= arr[target] {
        j -= 1;
    }
    arr.swap(target, j);

    arr[target + 1..].reverse();
    true
}

fn main() {
    let stdin = io::stdin();
    let mut hw = [[0; 2]; 6];
    let mut permutation: Vec<usize> = (0..6).collect();
    let mut rotate: usize;

    for i in 0..6 {
        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_err() {
            println!("no");
            return;
        }
        let nums: Vec<usize> = input.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() != 2 {
            println!("no");
            return;
        }
        hw[i][0] = nums[0];
        hw[i][1] = nums[1];
    }

    let connect: [[usize; 4]; 12] = [
        [0, 1, 1, 1],
        [0, 0, 2, 0],
        [0, 1, 3, 1],
        [0, 0, 4, 0],
        [1, 0, 2, 1],
        [2, 1, 3, 0],
        [3, 0, 4, 1],
        [4, 1, 1, 0],
        [5, 1, 1, 1],
        [5, 0, 4, 0],
        [5, 1, 3, 1],
        [5, 0, 4, 0],
    ];

    loop {
        for rotate in 0..(1 << 6) {
            let mut ng = false;
            for i in 0..12 {
                let len1 = hw[permutation[connect[i][0]]][connect[i][1] ^ ((rotate >> connect[i][0]) & 1)];
                let len2 = hw[permutation[connect[i][2]]][connect[i][3] ^ ((rotate >> connect[i][2]) & 1)];
                if len1 != len2 {
                    ng = true;
                    break;
                }
            }
            if !ng {
                println!("yes");
                return;
            }
        }
        if !next_permutation(&mut permutation) {
            break;
        }
    }
    println!("no");
}
