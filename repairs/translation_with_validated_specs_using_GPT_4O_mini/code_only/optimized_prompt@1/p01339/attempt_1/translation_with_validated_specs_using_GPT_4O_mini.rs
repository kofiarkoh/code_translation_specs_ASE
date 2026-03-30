// AOJ 2222 Alien's Counting
// 2018.4.25 bal4u

use std::io::{self, BufRead};
use std::ptr;

const MAX_V: usize = 1002;
static mut V: usize = 0; // 頂点数
static mut TO: [*mut i32; MAX_V] = [ptr::null_mut(); MAX_V];
static mut HI: [usize; MAX_V] = [0; MAX_V];
static mut LIM: [usize; MAX_V] = [0; MAX_V];
static mut R_TO: [*mut i32; MAX_V] = [ptr::null_mut(); MAX_V];
static mut R_HI: [usize; MAX_V] = [0; MAX_V];
static mut R_LIM: [usize; MAX_V] = [0; MAX_V];
static mut VS: [usize; MAX_V] = [0; MAX_V];
static mut VSZ: usize = 0;
static mut USED: [u8; MAX_V] = [0; MAX_V];
static mut CMP: [usize; MAX_V] = [0; MAX_V]; // 属する強連結成分のトポロジカル順序

unsafe fn check_lim(a: usize) {
    if HI[a] < LIM[a] {
        return;
    }
    if LIM[a] == 0 {
        LIM[a] = 8;
        HI[a] = 0;
        TO[a] = libc::malloc(std::mem::size_of::<i32>() * LIM[a]) as *mut i32;
    } else if HI[a] == LIM[a] {
        let mut k = LIM[a];
        if k > 256 {
            k = 256;
        }
        LIM[a] += k;
        TO[a] = libc::realloc(TO[a] as *mut libc::c_void, std::mem::size_of::<i32>() * LIM[a]) as *mut i32;
    }
}

unsafe fn check_r_lim(a: usize) {
    if R_HI[a] < R_LIM[a] {
        return;
    }
    if R_LIM[a] == 0 {
        R_LIM[a] = 8;
        R_HI[a] = 0;
        R_TO[a] = libc::malloc(std::mem::size_of::<i32>() * R_LIM[a]) as *mut i32;
    } else if R_HI[a] == R_LIM[a] {
        let mut k = R_LIM[a];
        if k > 256 {
            k = 256;
        }
        R_LIM[a] += k;
        R_TO[a] = libc::realloc(R_TO[a] as *mut libc::c_void, std::mem::size_of::<i32>() * R_LIM[a]) as *mut i32;
    }
}

unsafe fn add_edge(_from: usize, _to: usize) {
    check_lim(_from);
    check_r_lim(_to);
    *TO[_from].add(HI[_from]) = _to as i32;
    HI[_from] += 1;
    *R_TO[_to].add(R_HI[_to]) = _from as i32;
    R_HI[_to] += 1;
}

unsafe fn dfs(v: usize) {
    USED[v] = 1;
    for i in 0..HI[v] {
        if USED[*TO[v].add(i) as usize] == 0 {
            dfs(*TO[v].add(i) as usize);
        }
    }
    VS[VSZ] = v;
    VSZ += 1;
}

unsafe fn rdfs(v: usize, k: usize) {
    USED[v] = 1;
    CMP[v] = k;
    for i in 0..R_HI[v] {
        if USED[*R_TO[v].add(i) as usize] == 0 {
            rdfs(*R_TO[v].add(i) as usize, k);
        }
    }
}

unsafe fn init(n: usize) {
    V = n;
    std::ptr::write_bytes(LIM.as_mut_ptr(), 0, V);
    std::ptr::write_bytes(HI.as_mut_ptr(), 0, V);
    std::ptr::write_bytes(R_LIM.as_mut_ptr(), 0, V);
    std::ptr::write_bytes(R_HI.as_mut_ptr(), 0, V);
}

// 強連結成分への分解
unsafe fn scc() -> usize {
    let mut k = 0;
    std::ptr::write_bytes(USED.as_mut_ptr(), 0, V);
    VSZ = 0;
    for v in 0..V {
        if USED[v] == 0 {
            dfs(v);
        }
    }
    std::ptr::write_bytes(USED.as_mut_ptr(), 0, V);
    for i in (0..VSZ).rev() {
        if USED[VS[i]] == 0 {
            rdfs(VS[i], k);
            k += 1;
        }
    }
    k // 得られた強連結成分の数
}

// バッファを経ずstdinから数値を得る
fn in_number() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.bytes().next().unwrap().unwrap();
    while c >= b'0' {
        n = 10 * n + (c - b'0') as i32;
        c = handle.bytes().next().unwrap().unwrap();
    }
    n
}

const MOD: i32 = 1000000007;

fn main() {
    unsafe {
        let n = in_number();
        let m = in_number();
        V = n as usize;

        while m > 0 {
            let s = in_number() - 1;
            let d = in_number() - 1;
            add_edge(s as usize, d as usize);
            m -= 1;
        }

        let k = scc();
        let mut nx = vec![-1; k];
        for i in 0..n {
            if HI[i as usize] > 0 {
                nx[CMP[i as usize]] = CMP[*TO[i as usize].add(0) as usize];
            }
        }

        let mut ans: i64 = 1;
        let mut dp = vec![0; k];
        for i in 0..k {
            if dp[i] > 0 {
                dp[i] += 1;
            } else {
                dp[i] = 2;
            }
            if nx[i] < 0 || nx[i] == i {
                ans = ans * dp[i] as i64 % MOD as i64;
            } else if dp[nx[i] as usize] > 0 {
                dp[nx[i] as usize] = (dp[nx[i] as usize] as i64 * dp[i] as i64) % MOD as i64;
            } else {
                dp[nx[i] as usize] = dp[i];
            }
        }
        println!("{}", ans as i32);
    }
}
