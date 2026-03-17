/*
  AOJ 2157
  Title:
  @kankichi573
*/

use std::io::{self, BufRead};

static mut MIN: i32 = 99; // Using unsafe to maintain global mutable state like in C

fn find_not_zero(diff: &[u8], start: usize) -> i32 {
    for i in start..diff.len() {
        if diff[i] != 0 {
            return i as i32;
        }
    }
    -1
}

fn is_zero(diff: &[u8]) -> i32 {
    for &byte in diff {
        if byte != 0 {
            return 0;
        }
    }
    -1
}

fn dump(diff: &[u8]) {
    for &byte in diff {
        print!("{}|", byte);
    }
    println!();
}

fn rotate_count(diff: &mut [u8], acc: i32) {
    let mut pos = find_not_zero(diff, 0);

    if pos == -1 {
        unsafe {
            if MIN > acc {
                MIN = acc;
            }
        }
        return;
    }

    let offs = diff[pos as usize];
    let mut diff2 = diff.to_vec(); // Create a mutable copy of the diff array

    for i in pos as usize..diff.len() {
        diff2[i] = (diff2[i] + 10 - offs) % 10;
        rotate_count(&mut diff2, acc + 1);
    }
}

fn calc_diff(init: &[u8], unlock: &[u8], diff: &mut [u8]) {
    for i in 0..init.len() {
        let mut d = unlock[i] as i32 - init[i] as i32;
        if d < 0 {
            d += 10;
        }
        diff[i] = d as u8;
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = handle.lines();

    loop {
        let n: usize = match input.next() {
            Some(Ok(line)) => line.trim().parse().unwrap(),
            _ => break,
        };

        if n == 0 {
            break;
        }

        let mut init = vec![0; n];
        let mut unlock = vec![0; n];
        let mut diff = vec![0; n];

        let init_line = input.next().unwrap().unwrap();
        let unlock_line = input.next().unwrap().unwrap();

        for (i, byte) in init_line.bytes().enumerate() {
            if i < n {
                init[i] = byte;
            }
        }
        for (i, byte) in unlock_line.bytes().enumerate() {
            if i < n {
                unlock[i] = byte;
            }
        }

        if init == unlock {
            println!("0");
        } else {
            unsafe {
                MIN = 99; // Reset MIN for each case
            }
            calc_diff(&init, &unlock, &mut diff);
            rotate_count(&mut diff, 0);
            unsafe {
                println!("{}", MIN);
            }
        }
    }
}
