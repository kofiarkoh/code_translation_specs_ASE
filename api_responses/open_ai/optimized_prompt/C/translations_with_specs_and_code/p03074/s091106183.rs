use std::io::{self, Write};

const NUM_MAX: usize = 100000;
const BUF_SIZE: usize = NUM_MAX + 50;

fn get_int2() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a1 = iter.next().unwrap().parse::<usize>().unwrap();
    let a2 = iter.next().unwrap().parse::<usize>().unwrap();
    (a1, a2)
}

fn get_str(size: usize) -> String {
    let mut input = String::new();
    if size > 0 {
        io::stdin().read_line(&mut input).unwrap();
        input.truncate(size); // Ensure we only take the specified size
    }
    input
}

fn main() {
    let (num, changes) = get_int2();

    if num == 0 {
        println!("0");
        return;
    }

    let str_input = get_str(num);
    let mut str_vec: Vec<bool> = str_input.chars().map(|c| c == '1').collect();
    str_vec.push(!str_vec[num - 1]); // Append the negation of the last element

    let mut arr = vec![0; NUM_MAX * 4];
    let mut aidx = 0;
    let mut prev = true;
    let mut cnt = 0;

    for &value in &str_vec {
        if value == prev {
            cnt += 1;
            continue;
        }
        arr[aidx] = cnt;
        aidx += 1;
        prev = value;
        cnt = 1;
    }
    arr[aidx] = cnt; // Capture the last count
    aidx += 1; // Increment aidx to account for the last segment

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
