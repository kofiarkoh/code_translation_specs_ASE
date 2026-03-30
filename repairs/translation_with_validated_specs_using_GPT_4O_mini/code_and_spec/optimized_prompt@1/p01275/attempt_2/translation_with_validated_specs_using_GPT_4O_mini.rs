use std::io::{self, BufRead};

fn find_not_zero(diff: &[u8], start: usize) -> Option<usize> {
    for i in start..diff.len() {
        if diff[i] != 0 {
            return Some(i);
        }
    }
    None
}

fn is_zero(diff: &[u8]) -> bool {
    diff.iter().all(|&d| d == 0)
}

fn rotate_count(diff: &mut [u8], acc: usize, min_: &mut usize) {
    if let Some(pos) = find_not_zero(diff, 0) {
        let offs = diff[pos];
        let mut diff2 = diff.to_vec();
        for i in pos..diff.len() {
            diff2[i] = (diff2[i] + 10 - offs) % 10;
            rotate_count(&mut diff2, acc + 1, min_);
        }
    } else {
        if *min_ > acc {
            *min_ = acc;
        }
    }
}

fn calc_diff(init: &[u8], unlock: &[u8], diff: &mut [u8]) {
    for i in 0..init.len() {
        let d = (unlock[i] as i8 - init[i] as i8).rem_euclid(10) as u8;
        diff[i] = d;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let init = lines.next().unwrap().unwrap().trim().as_bytes().to_vec();
        let unlock = lines.next().unwrap().unwrap().trim().as_bytes().to_vec();

        if init == unlock {
            println!("0");
        } else {
            let mut min_ = usize::MAX;
            let mut diff = [0u8; 10];
            calc_diff(&init, &unlock, &mut diff);
            rotate_count(&mut diff, 0, &mut min_);
            println!("{}", min_);
        }
    }
}
