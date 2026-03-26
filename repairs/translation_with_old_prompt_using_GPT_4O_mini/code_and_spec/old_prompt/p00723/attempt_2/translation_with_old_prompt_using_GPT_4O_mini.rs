use std::io::{self, BufRead};

const MAX_STRINGS: usize = 1024;
const MAX_LENGTH: usize = 80;

static mut A: [[u8; MAX_LENGTH]; MAX_STRINGS] = [[0; MAX_LENGTH]; MAX_STRINGS];
static mut AP: usize = 0;

fn reverse(s: &mut [u8], n: usize) {
    let half = n / 2;
    for i in 0..half {
        s.swap(i, n - 1 - i);
    }
}

fn swap(s1: &mut [u8], s2: &mut [u8]) {
    let mut t = Vec::with_capacity(s1.len() + s2.len());
    t.extend_from_slice(s2);
    t.extend_from_slice(s1);
    let len = t.len();
    s1.copy_from_slice(&t[..len]);
}

fn contain(s: &[u8], len: usize) -> bool {
    unsafe {
        for i in 0..AP {
            if &A[i][..len] == s {
                return true;
            }
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        let mut s = lines.next().unwrap().unwrap().into_bytes();
        let len = s.len();
        let t = s.clone();

        unsafe {
            AP = 0;
        }

        for i in 1..len {
            unsafe {
                if !contain(&s[..len], len) {
                    A[AP].copy_from_slice(&s[..len]);
                    AP += 1;
                }
                reverse(&mut s, i);
                if !contain(&s[..len], len) {
                    A[AP].copy_from_slice(&s[..len]);
                    AP += 1;
                }
                reverse(&mut s[i..], len - i);
                if !contain(&s[..len], len) {
                    A[AP].copy_from_slice(&s[..len]);
                    AP += 1;
                }
                reverse(&mut s, i);
                if !contain(&s[..len], len) {
                    A[AP].copy_from_slice(&s[..len]);
                    AP += 1;
                }
                s.copy_from_slice(&t);
                
                // Create a separate mutable copy for swapping
                let mut s_copy = s.clone();
                swap(&mut s_copy, &mut s_copy[i..]);
                if !contain(&s_copy[..len], len) {
                    A[AP].copy_from_slice(&s_copy[..len]);
                    AP += 1;
                }
                reverse(&mut s_copy[len - i..], i);
                if !contain(&s_copy[..len], len) {
                    A[AP].copy_from_slice(&s_copy[..len]);
                    AP += 1;
                }
                reverse(&mut s_copy, len - i);
                if !contain(&s_copy[..len], len) {
                    A[AP].copy_from_slice(&s_copy[..len]);
                    AP += 1;
                }
                s.copy_from_slice(&t);
            }
        }
        println!("{}", unsafe { AP });
    }
}
