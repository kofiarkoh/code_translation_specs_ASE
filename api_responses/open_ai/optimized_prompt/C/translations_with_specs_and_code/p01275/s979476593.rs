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

fn rotate_count(diff: &mut [u8], acc: usize, min_ref: &mut usize) {
    if is_zero(diff) {
        *min_ref = (*min_ref).min(acc);
        return;
    }

    if let Some(pos) = find_not_zero(diff, 0) {
        let offs = diff[pos];
        let mut diff2 = diff.to_vec();

        for i in pos..diff.len() {
            diff2[i] = (diff2[i] + 10 - offs) % 10;
            rotate_count(&mut diff2, acc + 1, min_ref);
        }
    }
}

fn calc_diff(init: &[u8], unlock: &[u8], diff: &mut [u8]) {
    for i in 0..init.len() {
        let d = (unlock[i] as i32 - init[i] as i32 + 10) % 10;
        diff[i] = d as u8;
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

        let init: Vec<u8> = lines.next().unwrap().unwrap().trim().bytes().collect();
        let unlock: Vec<u8> = lines.next().unwrap().unwrap().trim().bytes().collect();

        if init == unlock {
            println!("0");
        } else {
            let mut min_rotations = usize::MAX;
            let mut diff = vec![0; n];
            calc_diff(&init, &unlock, &mut diff);
            rotate_count(&mut diff, 0, &mut min_rotations);
            println!("{}", min_rotations);
        }
    }
}
