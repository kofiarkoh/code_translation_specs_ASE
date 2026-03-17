// AOJ 0322: Slates
// 2019.2.24 bal4u

use std::io::{self, BufRead};
use std::cmp::Ordering;

fn gc() -> char {
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.bytes().next().unwrap().unwrap() as char
}

fn in_number() -> i32 {
    let mut n = 0;
    let mut c = gc();
    while c >= '0' && c <= '9' {
        n = 10 * n + (c as i32 - '0' as i32);
        c = gc();
    }
    n
}

fn ins(s: &mut [char], r: &mut [char]) -> usize {
    let mut ss = s.as_mut_ptr();
    let mut count = 0;

    loop {
        let c = gc();
        if c <= ' ' { break; }
        unsafe {
            *ss = c;
            ss = ss.add(1);
            count += 1;
        }
    }
    unsafe {
        *ss = '\0';
    }
    let n = count;
    for i in (0..n).rev() {
        r[i] = s[i];
    }
    r[n] = '\0';
    n
}

const MAX_N: usize = 50003;
const MAX_LEN: usize = 205;

static mut N: usize = 0;
static mut W: [[char; MAX_LEN]; MAX_N] = [['\0'; MAX_LEN]; MAX_N];
static mut R: [[char; MAX_LEN]; MAX_N] = [['\0'; MAX_LEN]; MAX_N];
static mut w: [&'static [char]; MAX_N] = [&[]; MAX_N];
static mut r: [&'static [char]; MAX_N] = [&[]; MAX_N];

fn cmp(a: &*const [char], b: &*const [char]) -> Ordering {
    let a_str = unsafe { std::ffi::CStr::from_ptr(*a as *const i8) };
    let b_str = unsafe { std::ffi::CStr::from_ptr(*b as *const i8) };
    a_str.cmp(b_str)
}

// Binary Search
fn search(s: &[char], n: usize, idx: usize, part: usize) -> usize {
    let mut low = 0;
    let mut high = unsafe { N };
    let mut k;

    if part != 0 {
        while low < high {
            let mid = (low + high) >> 1;
            if unsafe { strncmp(if idx != 0 { r[mid] } else { w[mid] }, s, n) <= 0 } {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        k = low - 1;
        low = 0;
        high = unsafe { N };
        while low < high {
            let mid = (low + high) >> 1;
            if unsafe { strncmp(if idx != 0 { r[mid] } else { w[mid] }, s, n) < 0 } {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return k - low + 1;
    } else {
        while low < high {
            let mid = (low + high) >> 1;
            k = unsafe { strcmp(if idx != 0 { r[mid] } else { w[mid] }, s) };
            if k == 0 {
                return 1;
            }
            if k < 0 {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
    }
    0
}

fn main() {
    unsafe {
        N = in_number() as usize;
        let M = in_number() as usize;
        let mut one = 0;

        for i in 0..N {
            let mut w_buf = ['\0'; MAX_LEN];
            let mut r_buf = ['\0'; MAX_LEN];
            if ins(&mut w_buf, &mut r_buf) == 1 {
                one += 1;
            }
            w[i] = &w_buf;
            r[i] = &r_buf;
        }

        w.sort_by(|a, b| cmp(a, b));
        r.sort_by(|a, b| cmp(a, b));

        for _ in 0..M {
            let mut slate = ['\0'; MAX_LEN];
            let mut res = ['\0'; MAX_LEN];
            let n = ins(&mut slate, &mut res);
            let k = slate.iter().position(|&c| c == '?').unwrap_or(n);
            let mut part = 0;
            let mut f_res = 0;
            let s: &mut [char] = if res[0] == '*' {
                slate[n - 1] = '\0';
                part = 1;
                &mut slate[..n - 1]
            } else if slate[0] == '*' {
                res[n - 1] = '\0';
                part = 1;
                f_res = 1;
                &mut res[..n - 1]
            } else {
                &mut slate[..n]
            };

            let ans = if k < n {
                if n == 1 {
                    if part != 0 { unsafe { N } } else { one }
                } else {
                    let mut ans = 0;
                    for i in 'a' as usize..='z' as usize {
                        s[k] = i as char;
                        ans += search(s, n, f_res, part);
                    }
                    ans
                }
            } else {
                search(s, n, f_res, part)
            };
            println!("{}", ans);
        }
    }
}

// Unsafe functions to mimic C string functions
unsafe fn strncmp(a: &[char], b: &[char], n: usize) -> i32 {
    for i in 0..n {
        if a[i] != b[i] {
            return a[i] as i32 - b[i] as i32;
        }
    }
    0
}

unsafe fn strcmp(a: &[char], b: &[char]) -> i32 {
    let mut i = 0;
    while a[i] != '\0' && b[i] != '\0' {
        if a[i] != b[i] {
            return a[i] as i32 - b[i] as i32;
        }
        i += 1;
    }
    a[i] as i32 - b[i] as i32
}
