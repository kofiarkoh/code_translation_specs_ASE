use std::io::{self, BufRead};

const MOD_BY: i64 = 1_000_000_007;

fn add(a: i64, b: i64) -> i64 {
    let mut r = a + b;
    if r >= MOD_BY {
        r -= MOD_BY;
    }
    r
}

struct Solver {
    a: Vec<i64>,
    b: Vec<i64>,
    c: Vec<i64>,
    n: usize,
    m: usize,
    exist_pos: usize,
    memo: [[[i64; 2]; 2]; 10002],
    number_of_number: [[i64; 2]; 2],
}

impl Solver {
    fn new(a: &str, b: &str, c: &str) -> Self {
        let alen = a.len();
        let blen = b.len();
        let mut a_vec = vec![0; blen];
        let mut b_vec = vec![0; blen];
        let mut c_vec = c.chars().map(|ch| (ch as i64) - ('0' as i64)).collect::<Vec<i64>>();

        // Pad A with zeros if necessary
        if alen < blen {
            for i in 0..alen {
                a_vec[blen - i - 1] = (a.chars().nth(alen - i - 1).unwrap() as i64) - ('0' as i64);
            }
            for i in 0..(blen - alen) {
                a_vec[i] = 0;
            }
        } else {
            a_vec.copy_from_slice(&a.chars().map(|ch| (ch as i64) - ('0' as i64)).collect::<Vec<i64>>());
        }

        let n = blen;
        let m = c_vec.len();
        let exist_pos = blen - alen;

        Self {
            a: a_vec,
            b: b.chars().map(|ch| (ch as i64) - ('0' as i64)).collect(),
            c: c_vec,
            n,
            m,
            exist_pos,
            memo: [[[0; 2]; 2]; 10002],
            number_of_number: [[0; 2]; 2],
        }
    }

    fn get_number_of_number(&mut self, pos: usize, ue_flag: bool, sita_flag: bool) -> i64 {
        if pos >= self.n {
            return 1;
        }
        if self.number_of_number[ue_flag as usize][sita_flag as usize] > 0 {
            return self.number_of_number[ue_flag as usize][sita_flag as usize] - 1;
        }

        let imin = if sita_flag { self.a[pos] } else { 0 };
        let imax = if ue_flag { self.b[pos] } else { 9 };
        let mut ret = 0;

        for i in imin..=imax {
            ret = add(ret, self.get_number_of_number(pos + 1, ue_flag && (i == imax), sita_flag && (i == imin)));
        }

        self.number_of_number[ue_flag as usize][sita_flag as usize] = ret + 1;
        ret
    }

    fn search(&mut self, pos: usize, seq_pos: usize, ue_flag: bool, sita_flag: bool) -> i64 {
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
    
    if input.len() != 3 {
        std::process::exit(1);
    }

    let a = &input[0];
    let b = &input[1];
    let c = &input[2];

    let mut solver = Solver::new(a, b, c);
    let result = solver.search(0, 0, true, true);
    println!("{}", result);
}
