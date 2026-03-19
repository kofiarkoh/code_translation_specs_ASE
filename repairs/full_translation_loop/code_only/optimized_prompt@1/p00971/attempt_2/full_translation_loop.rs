use std::io::{self, BufRead};

const SMAX: usize = 4000;
static mut NEXT0: [[usize; SMAX + 2]; 2] = [[0; SMAX + 2]; 2];
static mut NEXT1: [[usize; SMAX + 2]; 2] = [[0; SMAX + 2]; 2];
static mut DP: [[isize; SMAX + 2]; SMAX + 2] = [[-1; SMAX + 2]; SMAX + 2];

fn dfs(aloc: usize, bloc: usize) -> usize {
    unsafe {
        if DP[aloc][bloc] != -1 {
            return DP[aloc][bloc] as usize;
        }
        let mut ans = SMAX + 2;
        let res = dfs(NEXT0[0][aloc], NEXT0[1][bloc]);
        if ans > res {
            ans = res;
        }
        let res = dfs(NEXT1[0][aloc], NEXT1[1][bloc]);
        if ans > res {
            ans = res;
        }
        DP[aloc][bloc] = ans as isize + 1;
        DP[aloc][bloc] as usize
    }
}

fn scns(aloc: usize, bloc: usize) {
    unsafe {
        if dfs(aloc, bloc) == dfs(NEXT0[0][aloc], NEXT0[1][bloc]) + 1 {
            print!("0");
            scns(NEXT0[0][aloc], NEXT0[1][bloc]);
        } else if dfs(aloc, bloc) == dfs(NEXT1[0][aloc], NEXT1[1][bloc]) + 1 {
            print!("1");
            scns(NEXT1[0][aloc], NEXT1[1][bloc]);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut a = vec!['?'; SMAX + 2];
    let mut b = vec!['?'; SMAX + 2];

    handle.read_line(&mut a[1..]).unwrap();
    handle.read_line(&mut b[1..]).unwrap();

    let alen = a.iter().position(|&c| c == '\n').unwrap();
    let blen = b.iter().position(|&c| c == '\n').unwrap();

    unsafe {
        for row in DP.iter_mut() {
            row.fill(-1);
        }
        DP[alen][blen] = 0;

        NEXT0[0][alen] = alen;
        NEXT1[0][alen] = alen;
        NEXT0[0][alen - 1] = alen - 1;
        NEXT1[0][alen - 1] = alen - 1;
        NEXT0[1][blen] = blen;
        NEXT1[1][blen] = blen;
        NEXT0[1][blen - 1] = blen - 1;
        NEXT1[1][blen - 1] = blen - 1;

        for i in (0..alen - 1).rev() {
            if a[i + 1] == '0' {
                NEXT0[0][i] = i + 1;
                NEXT1[0][i] = NEXT1[0][i + 1];
            } else {
                NEXT0[0][i] = NEXT0[0][i + 1];
                NEXT1[0][i] = i + 1;
            }
        }

        for i in (0..blen - 1).rev() {
            if b[i + 1] == '0' {
                NEXT0[1][i] = i + 1;
                NEXT1[1][i] = NEXT1[1][i + 1];
            } else {
                NEXT0[1][i] = NEXT0[1][i + 1];
                NEXT1[1][i] = i + 1;
            }
        }
    }

    scns(0, 0);
    println!();
}
