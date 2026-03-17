use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut x = [0; 2];
    let mut y = [0; 2];
    let mut r = [0; 2];

    for i in 0..2 {
        let mut input = String::new();
        handle.read_line(&mut input).expect("Failed to read line");
        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        x[i] = nums[0];
        y[i] = nums[1];
        r[i] = nums[2];
    }

    let rdistance = (x[0] - x[1]).pow(2) + (y[0] - y[1]).pow(2);
    let rplus = (r[0] + r[1]).pow(2);
    let rminus = (r[0] - r[1]).pow(2);

    if rdistance > rplus {
        println!("4");
    } else if rdistance == rplus {
        println!("3");
    } else if rminus < rdistance && rdistance < rplus {
        println!("2");
    } else if rdistance == rminus {
        println!("1");
    } else if rdistance < rminus {
        println!("0");
    }
}
