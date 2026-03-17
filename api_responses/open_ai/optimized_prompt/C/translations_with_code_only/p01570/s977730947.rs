// AOJ 2456: Usoperanto
// 2018.4.17 bal4u

use std::io::{self, BufRead};
use std::ptr;

fn in_number() -> i32 {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();
    let mut chars = buffer.chars();
    let first_char = chars.next().unwrap();
    
    let mut n = 0;
    let mut is_negative = false;

    if first_char == '-' {
        is_negative = true;
        for c in chars {
            if c.is_digit(10) {
                n = n * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                break;
            }
        }
    } else {
        n = first_char.to_digit(10).unwrap() as i32;
        for c in chars {
            if c.is_digit(10) {
                n = n * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                break;
            }
        }
    }
    
    if is_negative {
        -n
    } else {
        n
    }
}

const MAX: usize = 1_000_002;
static mut N: usize = 0;
static mut M: [i32; MAX] = [0; MAX];
static mut BASE: *mut i32 = ptr::null_mut();
static mut B_SZ: usize = 0;
static mut HI: [usize; MAX] = [0; MAX];
static mut TO: [*mut i32; MAX] = [ptr::null_mut(); MAX];
static mut QUE: [usize; MAX] = [0; MAX];
static mut TOP: usize = 0;
static mut A: [usize; MAX] = [0; MAX];
static mut S: [i64; MAX] = [0; MAX];
static mut B: [i64; MAX] = [0; MAX];
static mut DP: [i64; MAX] = [0; MAX];

fn cmp(a: &i64, b: &i64) -> std::cmp::Ordering {
    if a == b {
        std::cmp::Ordering::Equal
    } else if a < b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Less
    }
}

fn calc(u: usize) {
    unsafe {
        let mut sz = 0;
        S[u] = M[u] as i64;
        DP[u] = 0;

        for i in 0..HI[u] {
            let v = TO[u][i];
            B[sz] = S[v];
            sz += 1;
            S[u] += S[v];
            DP[u] += DP[v];
        }

        B[0..sz].sort_by(|a, b| cmp(a, b));

        for i in 0..sz {
            DP[u] += B[i] * i as i64;
        }
    }
}

fn main() {
    unsafe {
        N = in_number() as usize;
        let mut memo: Vec<i32> = Vec::with_capacity(N);
        let mut bsz = 0;

        for i in 0..N {
            M[i] = in_number();
            let l = in_number();
            memo.push(l);
            if l == -1 {
                bsz += 1;
            } else {
                HI[l as usize] += 1;
            }
        }

        BASE = libc::malloc((bsz * std::mem::size_of::<i32>()) as libc::size_t) as *mut i32;

        for i in 0..N {
            if HI[i] > 0 {
                TO[i] = libc::malloc((HI[i] * std::mem::size_of::<i32>()) as libc::size_t) as *mut i32;
            }
        }

        bsz = 0;
        HI = [0; MAX];

        for i in 0..N {
            let l = memo[i];
            if l == -1 {
                *BASE.offset(bsz as isize) = i as i32;
                bsz += 1;
            } else {
                *TO[l as usize].offset(HI[l as usize] as isize) = i as i32;
                HI[l as usize] += 1;
            }
        }

        let mut ans = 0;
        for i in 0..bsz {
            QUE[1] = *BASE.offset(i as isize) as usize;
            TOP = 1;

            while TOP > 0 {
                let t = QUE[TOP];
                if A[t] < HI[t] {
                    TOP += 1;
                    QUE[TOP] = TO[t][A[t]];
                    A[t] += 1;
                } else {
                    calc(t);
                    TOP -= 1;
                }
            }
            ans += DP[*BASE.offset(i as isize) as usize];
        }
        println!("{}", ans);
    }
}
```

### Notes:
- The translation preserves the original C code's semantics, including handling of negative numbers and dynamic memory allocation.
- The `unsafe` block is used for raw pointer manipulation and memory allocation, which is necessary to maintain the same behavior as the original C code.
- The `cmp` function is adapted to match the original C comparison logic.
- The `in_number` function reads input and handles negative numbers similarly to the original C code.
- The Rust code uses `libc` for memory allocation to mimic C's `malloc`.
