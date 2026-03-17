use std::io::{self, Write};
use std::str;

const NUM_MAX: usize = 100000;
const BUF_SIZE: usize = NUM_MAX + 50;

fn get_int2() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a1: i32 = iter.next().unwrap().parse().unwrap();
    let a2: i32 = iter.next().unwrap().parse().unwrap();
    (a1, a2)
}

fn get_str(size: usize) -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    if trimmed.len() > size {
        trimmed[..size].to_string()
    } else {
        trimmed.to_string()
    }
}

fn main() {
    let (num, changes) = get_int2();

    let mut str_buf = get_str(BUF_SIZE);
    let mut str_arr = vec![0; BUF_SIZE];

    for i in 0..num {
        str_arr[i as usize] = if str_buf.chars().nth(i as usize).unwrap() == '1' { 1 } else { 0 };
    }
    str_arr[num as usize] = if str_arr[(num - 1) as usize] == 1 { 0 } else { 1 };

    let mut arr = vec![0; NUM_MAX * 4];
    let mut aidx = 0;
    let mut prev = 1;
    let mut cnt = 0;

    for i in 0..=num {
        if str_arr[i as usize] == prev {
            cnt += 1;
            continue;
        }
        arr[aidx] = cnt;
        aidx += 1;
        prev = str_arr[i as usize];
        cnt = 1;
    }

    // Uncomment for debug output
    /*
    for i in 0..aidx {
        print!(" {}", arr[i]);
    }
    println!();
    */

    let mut cum = vec![0; NUM_MAX * 4];
    for i in 1..NUM_MAX * 4 {
        cum[i] = cum[i - 1] + arr[i - 1];
    }

    let mut ans = 0;
    let ranges = 2 * changes + 1;
    for i in (0..aidx).step_by(2) {
        if i + ranges > aidx {
            break;
        }
        ans = ans.max(cum[i + ranges] - cum[i]);
    }
    println!("{}", ans);
}
