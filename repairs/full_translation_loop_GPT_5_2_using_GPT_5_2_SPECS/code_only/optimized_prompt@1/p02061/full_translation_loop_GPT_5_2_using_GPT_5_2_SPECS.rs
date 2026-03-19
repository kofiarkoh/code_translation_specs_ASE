use std::io::{self, Read};

static mut Q: i32 = 0;
static mut N: [i32; 114514] = [0; 114514];
static mut COUNT: [i32; 114514] = [0; 114514];

static mut MIZUKINANA: i32 = 0;
static mut FUKUENNMISATO: [[i32; 2]; 64] = [[0; 2]; 64];
static mut UTIYAMAYUMI: i64 = 0;
static mut YOSINAGASAYURI: i64 = 0;

fn minaseinori(idx: i32, acc: i64) {
    // This function relies on global mutable state exactly like the C code.
    // We keep the same recursion and arithmetic behavior.
    unsafe {
        if idx >= MIZUKINANA {
            if acc != UTIYAMAYUMI {
                // In C, signed overflow is UB; we emulate typical 2's-complement wrap
                // using wrapping arithmetic to preserve likely intended behavior.
                YOSINAGASAYURI = YOSINAGASAYURI.wrapping_mul(acc);

                // Clamp if greater than UTIYAMAYUMI * 2 (also with wrapping semantics).
                let limit = UTIYAMAYUMI.wrapping_mul(2);
                if YOSINAGASAYURI > limit {
                    YOSINAGASAYURI = limit;
                }
            }
        } else {
            let base = FUKUENNMISATO[idx as usize][0] as i64;
            let exp = FUKUENNMISATO[idx as usize][1];
            let mut a = acc;
            for _ in 0..=exp {
                minaseinori(idx + 1, a);
                a = a.wrapping_mul(base);
            }
        }
    }
}

fn main() {
    // Read all input, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let q: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => std::process::exit(1),
    };
    unsafe { Q = q };

    let mut max_n: i32 = 0;
    for i in 0..(q as usize) {
        let v: i32 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => std::process::exit(1),
        };
        unsafe { N[i] = v };
        if max_n < v {
            max_n = v;
        }
    }

    // COUNT[0] and COUNT[1] remain 0 as in C (global zero-initialized).
    for i in 2..=max_n {
        let mut sumipe: i32 = i;
        unsafe { MIZUKINANA = 0 };

        let mut tomatu: i32 = 2;
        // Use i64 for the square to avoid overflow in the loop condition.
        while (tomatu as i64) * (tomatu as i64) <= sumipe as i64 {
            if sumipe % tomatu == 0 {
                let mut c: i32 = 0;
                while sumipe % tomatu == 0 {
                    sumipe /= tomatu;
                    c += 1;
                }
                unsafe {
                    let idx = MIZUKINANA as usize;
                    FUKUENNMISATO[idx][0] = tomatu;
                    FUKUENNMISATO[idx][1] = c;
                    MIZUKINANA += 1;
                }
            }
            tomatu += 1;
        }

        if sumipe > 1 {
            unsafe {
                let idx = MIZUKINANA as usize;
                FUKUENNMISATO[idx][0] = sumipe;
                FUKUENNMISATO[idx][1] = 1;
                MIZUKINANA += 1;
            }
        }

        unsafe {
            UTIYAMAYUMI = i as i64;
            YOSINAGASAYURI = 1;
        }
        minaseinori(0, 1);

        unsafe {
            // In C: (yosinagasayuri >= i * 2) where i*2 is int arithmetic.
            // i is <= max_n from input; arrays bound suggests it fits in i32.
            let cond = (YOSINAGASAYURI >= (i as i64) * 2) as i32;
            COUNT[i as usize] = COUNT[(i - 1) as usize] + cond;
        }
    }

    let mut out = String::new();
    for i in 0..(q as usize) {
        let ni = unsafe { N[i] } as usize;
        let ans = unsafe { COUNT[ni] };
        out.push_str(&format!("{ans}\n"));
    }
    print!("{out}");
}
