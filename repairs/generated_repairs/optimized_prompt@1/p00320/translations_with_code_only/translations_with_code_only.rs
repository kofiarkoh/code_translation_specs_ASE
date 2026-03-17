use std::io::{self, BufRead};

fn next_permutation(arr: &mut [usize]) -> bool {
    let n = arr.len();
    let mut target = n as isize - 2;

    while target >= 0 {
        if arr[target as usize] < arr[(target + 1) as usize] {
            break;
        }
        target -= 1;
    }
    if target < 0 {
        return false;
    }

    let target = target as usize;

    let mut i = 0;
    while target + 1 + i < n - i - 1 {
        arr.swap(target + 1 + i, n - 1 - i);
        i += 1;
    }

    let mut i = target + 1;
    while i < n {
        if arr[i] > arr[target] {
            break;
        }
        i += 1;
    }
    arr.swap(i, target);
    true
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut hw = [[0; 2]; 6];
    for i in 0..6 {
        let mut line = String::new();
        if handle.read_line(&mut line).unwrap() == 0 {
            return;
        }
        let mut input = String::new();
        handle.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        hw[i][0] = parts.next().unwrap().parse().unwrap();
        hw[i][1] = parts.next().unwrap().parse().unwrap();
    }

    let mut permutation = [0, 1, 2, 3, 4, 5];
    loop {
        for rotate in 0..(1 << 6) {
            let mut ng = false;
            for i in 0..12 {
                let len1 = hw[permutation[CONNECT[i][0]]][CONNECT[i][1] ^ ((rotate >> CONNECT[i][0]) & 1)];
                let len2 = hw[permutation[CONNECT[i][2]]][CONNECT[i][3] ^ ((rotate >> CONNECT[i][2]) & 1)];
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

const CONNECT: [[usize; 4]; 12] = [
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
