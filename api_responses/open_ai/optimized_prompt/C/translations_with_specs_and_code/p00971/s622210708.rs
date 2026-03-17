use std::io::{self, BufRead};

const SMAX: usize = 4000;

struct DP {
    next0: [[usize; SMAX + 2]; 2],
    next1: [[usize; SMAX + 2]; 2],
    dp: [[i32; SMAX + 2]; SMAX + 2],
}

impl DP {
    fn new() -> Self {
        DP {
            next0: [[0; SMAX + 2]; 2],
            next1: [[0; SMAX + 2]; 2],
            dp: [[-1; SMAX + 2]; SMAX + 2],
        }
    }

    fn dfs(&mut self, aloc: usize, bloc: usize) -> i32 {
        if self.dp[aloc][bloc] != -1 {
            return self.dp[aloc][bloc];
        }
        let mut ans = SMAX as i32 + 2;
        let res = self.dfs(self.next0[0][aloc], self.next0[1][bloc]);
        if ans > res {
            ans = res;
        }
        let res = self.dfs(self.next1[0][aloc], self.next1[1][bloc]);
        if ans > res {
            ans = res;
        }
        self.dp[aloc][bloc] = ans + 1;
        self.dp[aloc][bloc]
    }

    fn scns(&mut self, aloc: usize, bloc: usize) {
        if self.dfs(aloc, bloc) == self.dfs(self.next0[0][aloc], self.next0[1][bloc]) + 1 {
            print!("0");
            self.scns(self.next0[0][aloc], self.next0[1][bloc]);
        } else if self.dfs(aloc, bloc) == self.dfs(self.next1[0][aloc], self.next1[1][bloc]) + 1 {
            print!("1");
            self.scns(self.next1[0][aloc], self.next1[1][bloc]);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();
    
    let mut dp = DP::new();
    
    let alen = a.len();
    let blen = b.len();
    
    dp.dp[alen][blen] = 0;
    dp.next0[0][alen] = alen;
    dp.next1[0][alen] = alen;
    dp.next0[0][alen - 1] = alen;
    dp.next1[0][alen - 1] = alen;
    dp.next0[1][blen] = blen;
    dp.next1[1][blen] = blen;
    dp.next0[1][blen - 1] = blen;
    dp.next1[1][blen - 1] = blen;

    for i in (0..alen - 1).rev() {
        if a.as_bytes()[i] == b'0' {
            dp.next0[0][i] = i + 1;
            dp.next1[0][i] = dp.next1[0][i + 1];
        } else {
            dp.next0[0][i] = dp.next0[0][i + 1];
            dp.next1[0][i] = i + 1;
        }
    }

    for i in (0..blen - 1).rev() {
        if b.as_bytes()[i] == b'0' {
            dp.next0[1][i] = i + 1;
            dp.next1[1][i] = dp.next1[1][i + 1];
        } else {
            dp.next0[1][i] = dp.next0[1][i + 1];
            dp.next1[1][i] = i + 1;
        }
    }

    dp.scns(0, 0);
    println!(); // Print newline at the end
}
