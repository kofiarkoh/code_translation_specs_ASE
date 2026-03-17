use std::io::{self, BufRead};

const MOD_BY: i32 = 1_000_000_007;

fn add(a: i32, b: i32) -> i32 {
    let mut r = a + b;
    if r >= MOD_BY {
        r -= MOD_BY;
    }
    r
}

struct Solver {
    a: Vec<i32>,
    b: Vec<i32>,
    c: Vec<i32>,
    n: usize,
    m: usize,
    exist_pos: usize,
    memo: [[[i32; 2]; 2]; 10002],
    number_of_number: [[i32; 2]; 2],
}

impl Solver {
    fn new(a: &str, b: &str, c: &str) -> Self {
        let alen = a.len();
        let blen = b.len();
        let mut a_vec = vec![0; blen];
        let mut b_vec = vec![0; blen];
        let mut c_vec = c.chars().map(|ch| ch.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();

        for (i, ch) in a.chars().enumerate() {
            a_vec[blen - alen + i] = ch.to_digit(10).unwrap() as i32;
        }
        for (i, ch) in b.chars().enumerate() {
            b_vec[i] = ch.to_digit(10).unwrap() as i32;
        }

        let exist_pos = blen - alen;

        Self {
            a: a_vec,
            b: b_vec,
            c: c_vec,
            n: blen,
            m: c_vec.len(),
            exist_pos,
            memo: [[[0; 2]; 2]; 10002],
            number_of_number: [[0; 2]; 2],
        }
    }

    fn get_number_of_number(&mut self, pos: usize, ue_flag: bool, sita_flag: bool) -> i32 {
        if pos >= self.n {
            return 1;
        }
        if self.number_of_number[pos][ue_flag as usize] > 0 {
            return self.number_of_number[pos][ue_flag as usize] - 1;
        }

        let imin = if sita_flag { self.a[pos] } else { 0 };
        let imax = if ue_flag { self.b[pos] } else { 9 };
        let mut ret = 0;

        for i in imin..=imax {
            ret = add(ret, self.get_number_of_number(pos + 1, ue_flag && (i == imax), sita_flag && (i == imin)));
        }

        self.number_of_number[pos][ue_flag as usize] = ret + 1;
        ret
    }

    fn search(&mut self, pos: usize, seq_pos: usize, ue_flag: bool, sita_flag: bool) -> i32 {
        if seq_pos >= self.m {
            return self.get_number_of_number(pos, ue_flag, sita_flag);
        }
        if pos >= self.n {
            return 0;
        }
        if self.memo[pos][seq_pos][ue_flag as usize][sita_flag as usize] > 0 {
            return self.memo[pos][seq_pos][ue_flag as usize][sita_flag as usize] - 1;
        }

        let imin = if sita_flag { self.a[pos] } else { 0 };
        let imax = if ue_flag { self.b[pos] } else { 9 };
        let mut ret = 0;

        for i in imin..=imax {
            if seq_pos == 0 {
                ret = add(ret, self.search(pos + 1, 0, ue_flag && (i == imax), sita_flag && (i == imin)));
            }
            if (!sita_flag || i != imin || self.exist_pos <= pos) && seq_pos < self.m && self.c[seq_pos] == i {
                ret = add(ret, self.search(pos + 1, seq_pos + 1, ue_flag && (i == imax), sita_flag && (i == imin)));
            }
        }

        self.memo[pos][seq_pos][ue_flag as usize][sita_flag as usize] = ret + 1;
        ret
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<_>, _>>().unwrap();

    if input.len() < 3 {
        std::process::exit(1);
    }

    let a = &input[0];
    let b = &input[1];
    let c = &input[2];

    let alen = a.len();
    let blen = b.len();
    let mut padded_a = a.clone();

    if alen < blen {
        let padding = "0".repeat(blen - alen);
        padded_a = format!("{}{}", padding, a);
    }

    let mut solver = Solver::new(&padded_a, b, c);
    let result = solver.search(0, 0, true, true);
    println!("{}", result);
}
