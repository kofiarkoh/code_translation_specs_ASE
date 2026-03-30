use std::io::{self, BufRead};
use std::mem;

const N: usize = 4000;
const L: usize = 62;
const K: usize = (N + L - 1) / L;
const MD: i64 = 998244353;

fn shift(aa: &mut [i64; K], bb: &mut [i64; K], s: usize) {
    let q = s / L;
    let r = s % L;
    bb.fill(0);
    for h in 0..K {
        if h + q >= K {
            break;
        }
        bb[h + q] |= (aa[h] & ((1 << (L - r)) - 1)) << r;
        if h + q + 1 < K && r > 0 {
            bb[h + q + 1] |= aa[h] >> (L - r);
        }
    }
}

fn xor(aa: &mut [i64; K], bb: &[i64; K]) {
    for h in 0..K {
        aa[h] ^= bb[h];
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut s = String::new();
    let mut aa = [0; K];
    let mut bb = [0; K];
    let mut cc = [0; K];
    let mut dd = [0; K];
    let mut p2 = [0; N];
    
    input.read_line(&mut s).unwrap();
    let mut m: i32 = s.trim().parse().unwrap();
    let n = s.trim().len();
    
    for (i, c) in s.trim().chars().enumerate() {
        if c == '1' {
            aa[(n - 1 - i) / L] |= 1 << (n - 1 - i) % L;
        }
    }
    
    while m > 0 {
        s.clear();
        input.read_line(&mut s).unwrap();
        let l = s.trim().len();
        bb.fill(0);
        
        for (i, c) in s.trim().chars().enumerate() {
            if c == '1' {
                bb[(l - 1 - i) / L] |= 1 << (l - 1 - i) % L;
            }
        }
        
        let mut i = N - 1;
        let mut j = N - 1;
        
        while i >= 0 || j >= 0 {
            while i >= 0 && (dd[i / L] & (1 << (i % L))) == 0 {
                i -= 1;
            }
            while j >= 0 && (bb[j / L] & (1 << (j % L))) == 0 {
                j -= 1;
            }
            if i < j {
                mem::swap(&mut dd, &mut bb);
                mem::swap(&mut i, &mut j);
            }
            if j < 0 {
                break;
            }
            shift(&bb, &mut cc, i - j);
            xor(&mut dd, &cc);
        }
        
        m -= 1;
    }
    
    let mut l = N;
    while l > 0 && (dd[(l - 1) / L] & (1 << ((l - 1) % L))) == 0 {
        l -= 1;
    }
    
    let mut p = 1;
    for i in 0..n {
        p2[i] = p;
        p = p * 2 % MD;
    }
    
    bb.fill(0);
    let mut ans = 0;
    
    for i in (0..n).rev() {
        let x = (aa[i / L] >> (i % L)) & 1;
        let y = (bb[i / L] >> (i % L)) & 1;

        if i >= l - 1 && x == 1 {
            ans = (ans + p2[i - l + 1]) % MD;
        }
        if x != y {
            if i >= l - 1 {
                shift(&mut dd, &mut cc, i - l + 1);
                xor(&mut bb, &cc);
            } else {
                if x > y {
                    ans = (ans + 1) % MD;
                }
                break;
            }
        }
    }
    
    if ans == 0 && (l == 0 || (dd[0] & 1) == 0) {
        ans = (ans + 1) % MD;
    }
    
    println!("{}", ans);
}
