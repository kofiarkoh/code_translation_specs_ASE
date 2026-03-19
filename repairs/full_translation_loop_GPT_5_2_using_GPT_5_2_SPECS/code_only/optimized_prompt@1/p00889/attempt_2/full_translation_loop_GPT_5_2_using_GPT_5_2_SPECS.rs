use std::io::{self, Read};

const HASHSIZ: usize = 300_017;

#[derive(Clone, Copy)]
struct HashEnt {
    b: i32,
    v: i32,
}

#[inline]
fn lookup(hash: &[HashEnt], b: i32) -> i32 {
    // Linear probing with wrap-around, identical to the C code.
    let mut idx = (b as u32 as usize) % HASHSIZ;
    loop {
        let e = hash[idx];
        if e.b == 0 {
            return 0;
        }
        if e.b == b {
            return e.v;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

#[inline]
fn insert(hash: &mut [HashEnt], b: i32) {
    let mut idx = (b as u32 as usize) % HASHSIZ;
    loop {
        let e = hash[idx];
        if e.b == 0 {
            hash[idx] = HashEnt { b, v: 1 };
            return;
        }
        if e.b == b {
            hash[idx].v += 1;
            return;
        }
        idx += 1;
        if idx == HASHSIZ {
            idx = 0;
        }
    }
}

fn main() {
    // Fast integer scanner (split_whitespace can be too slow and may cause TLE).
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut p = 0usize;
    let n_in = input.len();

    #[inline]
    fn next_i32(buf: &[u8], p: &mut usize) -> Option<i32> {
        let n = buf.len();
        while *p < n && buf[*p].is_ascii_whitespace() {
            *p += 1;
        }
        if *p >= n {
            return None;
        }
        let mut sign = 1i32;
        if buf[*p] == b'-' {
            sign = -1;
            *p += 1;
        }
        let mut x: i32 = 0;
        while *p < n {
            let c = buf[*p];
            if !c.is_ascii_digit() {
                break;
            }
            x = x.wrapping_mul(10).wrapping_add((c - b'0') as i32);
            *p += 1;
        }
        Some(x.wrapping_mul(sign))
    }

    let mut tr = [[0u8; 6]; 10];
    tr[0][2] = 1;
    tr[2][2] = 1;
    tr[4][2] = 1;
    tr[6][2] = 1;
    tr[8][2] = 1;
    tr[0][5] = 1;
    tr[5][5] = 1;

    // Match C array sizes.
    let mut a = vec![0i32; 100_002];
    let mut b = vec![0i32; 100_003]; // need b[n] and b[i+1]
    let mut hash = vec![HashEnt { b: 0, v: 0 }; HASHSIZ + 2];

    let mut out = String::new();

    while p < n_in {
        let n = match next_i32(&input, &mut p) {
            Some(v) => v,
            None => break,
        };
        let s = match next_i32(&input, &mut p) {
            Some(v) => v,
            None => break,
        };
        let w = match next_i32(&input, &mut p) {
            Some(v) => v,
            None => break,
        };
        let q = match next_i32(&input, &mut p) {
            Some(v) => v,
            None => break,
        };

        if n <= 0 {
            break;
        }
        let n_us = n as usize;

        // Generate a[]
        let mut g: i32 = s;
        for i in 0..n_us {
            a[i] = (g / 7) % 10;
            if (g & 1) != 0 {
                g = (g >> 1) ^ w;
            } else {
                g >>= 1;
            }
        }

        let mut ans: i32 = 0;

        if q == 2 || q == 5 {
            let mut k: i32 = 0;
            for i in 0..n_us {
                if a[i] != 0 {
                    k += 1;
                }
                if tr[a[i] as usize][q as usize] != 0 {
                    ans = ans.wrapping_add(k);
                }
            }
        } else {
            // Clear hash table (same as memset to 0).
            hash.fill(HashEnt { b: 0, v: 0 });

            // Compute suffix remainders b[i]
            let mut k: i32 = 1;
            b[n_us] = 0;
            for i in (0..n_us).rev() {
                // In C, % with positive q yields non-negative remainder.
                let tmp = (k.wrapping_mul(a[i]).wrapping_add(b[i + 1])) % q;
                b[i] = tmp;
                // k = (k*10) % q
                let k10 = (k.wrapping_shl(3)).wrapping_add(k.wrapping_shl(1));
                k = k10 % q;
            }

            for i in 0..n_us {
                if a[i] != 0 {
                    insert(&mut hash, b[i] + 1);
                }
                ans = ans.wrapping_add(lookup(&hash, b[i + 1] + 1));
            }
        }

        out.push_str(&ans.to_string());
        out.push('\n');
    }

    print!("{out}");
}
