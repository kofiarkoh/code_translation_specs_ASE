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
    if size == 0 {
        return input;
    }
    let format = format!("{{:width$}}", "", width = size);
    io::stdin().read_line(&mut input).unwrap();
    input.truncate(size); // Ensure we only take the first `size` characters
    input
}

fn main() {
    let (num, changes) = get_int2();

    let str_input = get_str(BUF_SIZE);
    let mut str_vec: Vec<bool> = vec![false; BUF_SIZE];
    
    for i in 0..num {
        str_vec[i as usize] = str_input.chars().nth(i as usize).unwrap() == '1';
    }
    str_vec[num as usize] = !str_vec[(num - 1) as usize];

    // #1 #0 #1 #0...
    let mut arr = vec![0; NUM_MAX * 4];
    let mut aidx = 0;
    let mut prev = true;
    let mut cnt = 0;

    for i in 0..=num {
        if str_vec[i as usize] == prev {
            cnt += 1;
            continue;
        }
        arr[aidx] = cnt;
        aidx += 1;
        prev = str_vec[i as usize];
        cnt = 1;
    }

    // Cumulative sum array
    let mut cum = vec![0; NUM_MAX * 4];
    for i in 1..(NUM_MAX * 4) {
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
