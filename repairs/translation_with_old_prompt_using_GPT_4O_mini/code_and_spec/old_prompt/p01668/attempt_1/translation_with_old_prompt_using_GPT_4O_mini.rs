use std::io::{self, BufRead};

const MOD_BY: i32 = 1_000_000_007;

fn add(a: i32, b: i32) -> i32 {
    let mut r = a + b;
    if r >= MOD_BY {
        r -= MOD_BY;
    }
    r
}

fn get_number_of_number(pos: usize, ue_flag: bool, sita_flag: bool, a: &[i32], b: &[i32], memo: &mut Vec<Vec<Vec<Vec<i32>>>) -> i32 {
    if pos >= a.len() {
        return 1;
    }
    if memo[pos][ue_flag as usize][sita_flag as usize] > 0 {
        return memo[pos][ue_flag as usize][sita_flag as usize] - 1;
    }

    let imin = if sita_flag { a[pos] } else { 0 };
    let imax = if ue_flag { b[pos] } else { 9 };
    let mut ret = 0;

    for i in imin..=imax {
        ret = add(ret, get_number_of_number(pos + 1, ue_flag && (i == imax), sita_flag && (i == imin), a, b, memo));
    }

    memo[pos][ue_flag as usize][sita_flag as usize] = ret + 1;
    ret
}

fn search(pos: usize, seq_pos: usize, ue_flag: bool, sita_flag: bool, a: &[i32], b: &[i32], c: &[i32], memo: &mut Vec<Vec<Vec<Vec<i32>>>) -> i32 {
    if seq_pos >= c.len() {
        return get_number_of_number(pos, ue_flag, sita_flag, a, b, memo);
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
        if (!sita_flag || i != imin || pos >= a.len()) && seq_pos < c.len() && c[seq_pos] == i {
            ret = add(ret, search(pos + 1, seq_pos + 1, ue_flag && (i == imax), sita_flag && (i == imin), a, b, c, memo));
        }
    }

    memo[pos][seq_pos][ue_flag as usize][sita_flag as usize] = ret + 1;
    ret
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<_>, _>>().unwrap();
    
    if input.len() != 3 {
        std::process::exit(1);
    }

    let a_str = input[0].trim();
    let b_str = input[1].trim();
    let c_str = input[2].trim();

    let alen = a_str.len();
    let blen = b_str.len();
    let mut a = vec![0; blen];
    let mut b = vec![0; blen];
    let mut c = vec![0; c_str.len()];

    if alen < blen {
        for i in 0..alen {
            a[blen - i - 1] = a_str.chars().nth(alen - i - 1).unwrap().to_digit(10).unwrap() as i32;
        }
        for i in 0..(blen - alen) {
            a[i] = 0;
        }
    } else {
        for i in 0..blen {
            a[i] = a_str.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
        }
    }

    for i in 0..blen {
        b[i] = b_str.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
    }

    for i in 0..c_str.len() {
        c[i] = c_str.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
    }

    let n = blen;
    let m = c.len();
    let mut memo = vec![vec![vec![vec![0; 2]; 2]; m + 1]; n + 1];

    let result = search(0, 0, true, true, &a, &b, &c, &mut memo);
    println!("{}", result);
}
