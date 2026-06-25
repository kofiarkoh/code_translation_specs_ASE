use std::io::{self, Read};

const MOD: i32 = 1_000_000_007;

fn add_mod(a: i32, b: i32) -> i32 {
    let mut r = a + b;
    if r >= MOD {
        r -= MOD;
    }
    r
}

struct Solver {
    n: usize,
    m: usize,
    exist_pos: usize,
    a: Vec<u8>, // digits 0..9, length n (lower bound padded)
    b: Vec<u8>, // digits 0..9, length n (upper bound)
    c: Vec<u8>, // digits 0..9, length m (pattern)

    // memoization with "ret+1" sentinel (0 means uncomputed)
    memo: Vec<i32>,      // [n+1][m+1][2][2]
    num_memo: Vec<i32>,  // [n+1][2][2]
}

impl Solver {
    fn idx_memo(&self, pos: usize, seq_pos: usize, ue: usize, sita: usize) -> usize {
        // dims: (n+1) * (m+1) * 2 * 2
        (((pos * (self.m + 1) + seq_pos) * 2 + ue) * 2 + sita)
    }
    fn idx_num(&self, pos: usize, ue: usize, sita: usize) -> usize {
        // dims: (n+1) * 2 * 2
        ((pos * 2 + ue) * 2 + sita)
    }

    fn get_number_of_number(&mut self, pos: usize, ue_flag: bool, sita_flag: bool) -> i32 {
        if pos >= self.n {
            return 1;
        }
        let ue = ue_flag as usize;
        let sita = sita_flag as usize;
        let idx = self.idx_num(pos, ue, sita);
        let cached = self.num_memo[idx];
        if cached > 0 {
            return cached - 1;
        }

        let imin = if sita_flag { self.a[pos] } else { 0 };
        let imax = if ue_flag { self.b[pos] } else { 9 };

        let mut ret: i32 = 0;
        for d in imin..=imax {
            let next_ue = ue_flag && d == imax;
            let next_sita = sita_flag && d == imin;
            ret = add_mod(ret, self.get_number_of_number(pos + 1, next_ue, next_sita));
        }

        self.num_memo[idx] = ret + 1;
        ret
    }

    fn search(&mut self, pos: usize, seq_pos: usize, ue_flag: bool, sita_flag: bool) -> i32 {
        if seq_pos >= self.m {
            return self.get_number_of_number(pos, ue_flag, sita_flag);
        }
        if pos >= self.n {
            return 0;
        }

        let ue = ue_flag as usize;
        let sita = sita_flag as usize;
        let idx = self.idx_memo(pos, seq_pos, ue, sita);
        let cached = self.memo[idx];
        if cached > 0 {
            return cached - 1;
        }

        let imin = if sita_flag { self.a[pos] } else { 0 };
        let imax = if ue_flag { self.b[pos] } else { 9 };

        let mut ret: i32 = 0;
        for d in imin..=imax {
            let next_ue = ue_flag && d == imax;
            let next_sita = sita_flag && d == imin;

            // Option to not start/continue the pattern (only when seq_pos==0)
            if seq_pos == 0 {
                ret = add_mod(ret, self.search(pos + 1, 0, next_ue, next_sita));
            }

            // Option to match next pattern digit, with the "existPos" restriction.
            // Mirrors C condition:
            // (!sitaFlag || i!=imin || existPos<=pos) && C[seqPos]==i
            let allowed_start_here = (!sita_flag) || (d != imin) || (self.exist_pos <= pos);
            if allowed_start_here && self.c[seq_pos] == d {
                ret = add_mod(ret, self.search(pos + 1, seq_pos + 1, next_ue, next_sita));
            }
        }

        self.memo[idx] = ret + 1;
        ret
    }
}

fn main() {
    // Read all input; if not exactly 3 tokens, exit non-zero with no defined output.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();
    let a_str = match it.next() {
        Some(s) => s.to_string(),
        None => std::process::exit(1),
    };
    let b_str = match it.next() {
        Some(s) => s.to_string(),
        None => std::process::exit(1),
    };
    let c_str = match it.next() {
        Some(s) => s.to_string(),
        None => std::process::exit(1),
    };

    let alen = a_str.len();
    let blen = b_str.len();

    // Pad lower bound with leading zeros to match upper bound length.
    let mut a_padded = if alen < blen {
        let mut s = String::with_capacity(blen);
        for _ in 0..(blen - alen) {
            s.push('0');
        }
        s.push_str(&a_str);
        s
    } else {
        a_str
    };

    // Note: Spec precondition says alen <= blen; if violated, behavior is undefined.
    // We follow the C code behavior (no truncation), but still proceed.

    let n = blen;
    let m = c_str.len();
    let exist_pos = blen.saturating_sub(alen);

    // Convert to digit arrays (assumes only '0'..'9' per preconditions).
    let mut a_digits = Vec::with_capacity(n);
    for ch in a_padded.drain(..).take(n) {
        a_digits.push((ch as u8).wrapping_sub(b'0'));
    }
    let mut b_digits = Vec::with_capacity(n);
    for ch in b_str.bytes().take(n) {
        b_digits.push(ch - b'0');
    }
    let mut c_digits = Vec::with_capacity(m);
    for ch in c_str.bytes() {
        c_digits.push(ch - b'0');
    }

    let memo = vec![0i32; (n + 1) * (m + 1) * 2 * 2];
    let num_memo = vec![0i32; (n + 1) * 2 * 2];

    let mut solver = Solver {
        n,
        m,
        exist_pos,
        a: a_digits,
        b: b_digits,
        c: c_digits,
        memo,
        num_memo,
    };

    let ans = solver.search(0, 0, true, true);
    print!("{ans}\n");
}
