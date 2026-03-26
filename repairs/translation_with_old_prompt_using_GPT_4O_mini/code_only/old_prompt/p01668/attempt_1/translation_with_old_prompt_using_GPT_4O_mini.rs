use std::io::{self, BufRead};

const MOD_BY: i32 = 1000000007;

fn add(a: i32, b: i32) -> i32 {
    let mut r = a + b;
    if r >= MOD_BY {
        r -= MOD_BY;
    }
    r
}

fn get_number_of_number(pos: usize, ue_flag: bool, sita_flag: bool, a: &[i32], b: &[i32], number_of_number: &mut [[[i32; 2]; 2]; 10002]) -> i32 {
    if pos >= a.len() {
        return 1;
    }
    if number_of_number[pos][ue_flag as usize][sita_flag as usize] > 0 {
        return number_of_number[pos][ue_flag as usize][sita_flag as usize] - 1;
    }

    let imin = if sita_flag { a[pos] } else { 0 };
    let imax = if ue_flag { b[pos] } else { 9 };
    let mut ret = 0;

    for i in imin..=imax {
        ret = add(ret, get_number_of_number(pos + 1, ue_flag && (i == imax), sita_flag && (i == imin), a, b, number_of_number));
    }

    number_of_number[pos][ue_flag as usize][sita_flag as usize] = ret + 1;
    ret
}

fn search(pos: usize, seq_pos: usize, ue_flag: bool, sita_flag: bool, a: &[i32], b: &[i32], c: &[i32], memo: &mut [[[[i32; 2]; 2]; 502]; 10002]) -> i32 {
    if seq_pos >= c.len() {
        return get_number_of_number(pos, ue_flag, sita_flag, a, b, &mut [[0; 2]; 2]);
    }
    if pos >= a.len() {
        return 0;
    }
    if memo[pos][seq_pos][ue_flag as usize][sita_flag as usize] > 0 {
        return memo[pos][seq_pos][ue_flag as usize][sita_flag as usize] - 1;
    }

    let imin = if sita_flag { a[pos] } else { 0 };
    let imax = if ue_flag { b[pos] } else { 9 };
    let mut ret = 0;

    for i in imin..=imax {
        if seq_pos == 0 {
            ret = add(ret, search(pos + 1, 0, ue_flag && (i == imax), sita_flag && (i == imin), a, b, c, memo));
        }
        if (!sita_flag || i != imin || exist_pos <= pos) && seq_pos < c.len() && c[seq_pos] == i {
            ret = add(ret, search(pos + 1, seq_pos + 1, ue_flag && (i == imax), sita_flag && (i == imin), a, b, c, memo));
        }
    }

    memo[pos][seq_pos][ue_flag as usize][sita_flag as usize] = ret + 1;
    ret
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<_>, _>>().unwrap();
    let a = input[0].trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();
    let b = input[1].trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();
    let c = input[2].trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();

    let alen = a.len();
    let blen = b.len();
    let mut a = a.clone();
    let mut b = b.clone();
    let mut c = c.clone();

    if alen < blen {
        for i in 0..alen {
            a[blen - i - 1] = a[alen - i - 1];
        }
        for i in 0..(blen - alen) {
            a[i] = 0;
        }
    }

    let n = blen;
    let m = c.len();
    let exist_pos = blen - alen;

    let mut memo = [[[[0; 2]; 2]; 502]; 10002];
    for i in 0..n {
        a[i] -= 0; // No need to change, just for clarity
        b[i] -= 0; // No need to change, just for clarity
    }
    for i in 0..m {
        c[i] -= 0; // No need to change, just for clarity
    }

    println!("{}", search(0, 0, true, true, &a, &b, &c, &mut memo));
}
