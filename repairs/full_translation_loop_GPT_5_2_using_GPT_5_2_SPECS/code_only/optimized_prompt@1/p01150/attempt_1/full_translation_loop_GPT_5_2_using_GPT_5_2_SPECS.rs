use std::io::{self, Read};

const FACT8: u64 = 40320;
const N_OF_PRINCES: usize = 8;

// Mirrors the C globals.
static mut N: i32 = 0;
static mut RESULT: [i32; 70] = [0; 70];
static mut MEMO: [[[[u32; 3]; 3]; 9]; 70] = [[[[0u32; 3]; 3]; 9]; 70];

// DEBUG dump() is compiled out in the C code unless DEBUG is defined.
// We keep a no-op to preserve structure without affecting I/O.
#[allow(dead_code)]
fn dump(_f: i32) {}

fn calc_odd_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize) -> u32 {
    // Uses global memo/result/N like the C code.
    unsafe {
        let cached = MEMO[pos][princes][sitdown][zero_sitdown];
        if cached != 0 {
            return cached;
        }

        let n = N as usize;
        let mut ret: u32;

        if princes == 0 {
            dump(0);
            ret = 1;
        } else if pos == n - 1 && princes > 1 {
            ret = 0;
        } else if pos == n - 1 && princes == 1 {
            if zero_sitdown == 0 && sitdown == 0 {
                dump(1);
                ret = 1;
            } else {
                ret = 0;
            }
        } else {
            if sitdown == 0 {
                RESULT[pos] = 1;
                ret = calc_odd_1(pos + 1, princes - 1, 1, zero_sitdown);
            } else {
                ret = 0;
            }

            RESULT[pos] = 0;
            ret = ret.wrapping_add(calc_odd_1(pos + 1, princes, 0, zero_sitdown));
        }

        MEMO[pos][princes][sitdown][zero_sitdown] = ret;
        ret
    }
}

fn calc_odd(_n: i32) -> u32 {
    unsafe {
        let mut ret: u32 = 0;

        RESULT[0] = 1;
        ret = ret.wrapping_add(calc_odd_1(1, N_OF_PRINCES - 1, 1, 1));
        RESULT[0] = 0;
        ret = ret.wrapping_add(calc_odd_1(1, N_OF_PRINCES, 0, 0));
        ret
    }
}

fn calc_even_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize) -> u32 {
    unsafe {
        let cached = MEMO[pos][princes][sitdown][zero_sitdown];
        if cached != 0 {
            return cached;
        }

        let n = N as usize;
        let half = n / 2;
        let mut ret: u32;

        if princes == 0 {
            dump(0);
            ret = 1;
        } else if pos == half - 1 && princes > 1 {
            ret = 0;
        } else if pos == half - 1 && princes == 1 {
            if zero_sitdown == 0 && sitdown == 0 {
                dump(2);
                ret = 2;
            } else if (zero_sitdown == 2 || zero_sitdown == 0) && (sitdown == 1 || sitdown == 0) {
                dump(3);
                ret = 1;
            } else if (zero_sitdown == 1 || zero_sitdown == 0) && (sitdown == 2 || sitdown == 0) {
                dump(4);
                ret = 1;
            } else {
                ret = 0;
            }
        } else {
            ret = 0;

            if sitdown == 0 || sitdown == 2 {
                RESULT[pos] = 1;
                RESULT[pos + half] = 0;
                ret = ret.wrapping_add(calc_even_1(pos + 1, princes - 1, 1, zero_sitdown));
            }
            if sitdown == 0 || sitdown == 1 {
                RESULT[pos] = 0;
                RESULT[pos + half] = 1;
                ret = ret.wrapping_add(calc_even_1(pos + 1, princes - 1, 2, zero_sitdown));
            }

            RESULT[pos] = 0;
            RESULT[pos + half] = 0;
            ret = ret.wrapping_add(calc_even_1(pos + 1, princes, 0, zero_sitdown));
        }

        MEMO[pos][princes][sitdown][zero_sitdown] = ret;
        ret
    }
}

fn calc_even(_n: i32) -> u32 {
    unsafe {
        let n = N as usize;
        let half = n / 2;
        let mut ret: u32 = 0;

        RESULT[0] = 1;
        RESULT[half] = 0;
        ret = ret.wrapping_add(calc_even_1(1, N_OF_PRINCES - 1, 1, 1)); // zenhan

        RESULT[0] = 0;
        RESULT[half] = 1;
        ret = ret.wrapping_add(calc_even_1(1, N_OF_PRINCES - 1, 2, 2)); // kouhan

        RESULT[0] = 0;
        RESULT[half] = 0;
        ret = ret.wrapping_add(calc_even_1(1, N_OF_PRINCES, 0, 0)); // none

        ret
    }
}

fn main() {
    // Read all input and parse ints like repeated scanf("%d",&N).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    while let Some(tok) = it.next() {
        let n_val: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if n_val == 0 {
            break;
        }

        // The original C code assumes N is within bounds for memo/result (<=70).
        // We preserve that assumption; out-of-range would be UB in C anyway.
        unsafe {
            N = n_val;
            MEMO = [[[[0u32; 3]; 3]; 9]; 70];
        }

        let ret: u64 = if n_val <= 16 {
            0
        } else if (n_val & 1) != 0 {
            FACT8.wrapping_mul(calc_odd(n_val) as u64)
        } else {
            FACT8.wrapping_mul(calc_even(n_val) as u64)
        };

        out.push_str(&format!("{}\n", ret as i64));
    }

    print!("{out}");
}
