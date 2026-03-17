use std::io::{self, Read};

const MOD_BY: i32 = 1000000007;

fn add(a: i32, b: i32) -> i32 {
    let mut r = a + b;
    if r >= MOD_BY {
        r -= MOD_BY;
    }
    r
}

fn get_number_of_number(pos: usize, ue_flag: bool, sita_flag: bool, a: &[i32], b: &[i32], number_of_number: &mut [[[i32; 2]; 2]; 10002], n: usize) -> i32 {
    if pos >= n {
        return 1;
    }
    if number_of_number[pos][ue_flag as usize][sita_flag as usize] > 0 {
        return number_of_number[pos][ue_flag as usize][sita_flag as usize] - 1;
    }

    let imin = if sita_flag { a[pos] } else { 0 };
    let imax = if ue_flag { b[pos] } else { 9 };
    let mut ret = 0;

    for i in imin..=imax {
        ret = add(ret, get_number_of_number(pos + 1, ue_flag && (i == imax), sita_flag && (i == imin), a, b, number_of_number, n));
    }

    number_of_number[pos][ue_flag as usize][sita_flag as usize] = ret + 1;
    ret
}

fn search(pos: usize, seq_pos: usize, ue_flag: bool, sita_flag: bool, a: &[i32], b: &[i32], c: &[i32], memo: &mut [[[[i32; 2]; 2]; 502]; 10002], number_of_number: &mut [[[i32; 2]; 2]; 10002], n: usize, m: usize, exist_pos: usize) -> i32 {
    if seq_pos >= m {
        return get_number_of_number(pos, ue_flag, sita_flag, a, b, number_of_number, n);
    }
    if pos >= n {
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
            ret = add(ret, search(pos + 1, 0, ue_flag && (i == imax), sita_flag && (i == imin), a, b, c, memo, number_of_number, n, m, exist_pos));
        }
        if (!sita_flag || i != imin || exist_pos <= pos) && seq_pos < m && c[seq_pos] == i {
            ret = add(ret, search(pos + 1, seq_pos + 1, ue_flag && (i == imax), sita_flag && (i == imin), a, b, c, memo, number_of_number, n, m, exist_pos));
        }
    }

    memo[pos][seq_pos][ue_flag as usize][sita_flag as usize] = ret + 1;
    ret
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let a = lines.next().unwrap();
    let b = lines.next().unwrap();
    let c = lines.next().unwrap();

    let alen = a.len();
    let blen = b.len();
    let mut a_vec = vec![0; 10240];
    let mut b_vec = vec![0; 10240];
    let mut c_vec = vec![0; 512];

    let mut exist_pos = 0;

    if alen < blen {
        for i in 0..alen {
            a_vec[blen - i - 1] = a.chars().nth(alen - i - 1).unwrap().to_digit(10).unwrap() as i32;
        }
        for i in 0..(blen - alen) {
            a_vec[i] = 0;
        }
        exist_pos = blen - alen;
    }

    let n = blen;
    let m = c.len();

    for i in 0..n {
        a_vec[i] = a.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
        b_vec[i] = b.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
    }
    for i in 0..m {
        c_vec[i] = c.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
    }

    let mut memo = [[[[0; 2]; 2]; 502]; 10002];
    let mut number_of_number = [[[0; 2]; 2]; 10002];

    println!("{}", search(0, 0, true, true, &a_vec, &b_vec, &c_vec, &mut memo, &mut number_of_number, n, m, exist_pos));
}
