use std::io::{self, BufRead};

fn next_permutation(arr: &mut [usize]) -> i32 {
    let n = arr.len();
    let mut target = -1;

    for i in (1..n).rev() {
        if arr[i - 1] < arr[i] {
            target = (i - 1) as i32;
            break;
        }
    }
    if target < 0 {
        return 0;
    }

    let target = target as usize;

    for i in 0..(target + 1).min(n - target - 1) {
        arr.swap(target + 1 + i, n - 1 - i);
    }

    let mut i = target + 1;
    while i < n && arr[i] <= arr[target] {
        i += 1;
    }
    arr.swap(i, target);
    1
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut hw = [[0; 2]; 6];
    let mut permutation = [0, 1, 2, 3, 4, 5];

    for i in 0..6 {
        let mut input = String::new();
        handle.read_line(&mut input).unwrap();
        let nums: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
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
            let mut ng = 0;
            for i in 0..12 {
                let len1 = hw[permutation[connect[i][0]]][connect[i][1] ^ ((rotate >> connect[i][0]) & 1)];
                let len2 = hw[permutation[connect[i][2]]][connect[i][3] ^ ((rotate >> connect[i][2]) & 1)];
                if len1 != len2 {
                    ng = 1;
                    break;
                }
            }
            if ng == 0 {
                println!("yes");
                return;
            }
        }
        if next_permutation(&mut permutation) == 0 {
            break;
        }
    }
    println!("no");
}
