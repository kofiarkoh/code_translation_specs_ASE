use std::io::{self, Read};

const FACT8: i64 = 40320;
const N_OF_PRINCES: usize = 8;

// Mirrors the C globals.
static mut N: i32 = 0;
static mut RESULT: [i32; 70] = [0; 70];
static mut MEMO: [[[[u32; 3]; 3]; 9]; 70] = [[[[0u32; 3]; 3]; 9]; 70];

#[allow(dead_code)]
fn dump(_f: i32) {
    // DEBUG-only in the C code; intentionally no-op to preserve non-DEBUG behavior.
}

fn calc_odd_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize) -> u32 {
    // All indices are within bounds by construction (pos < 70, princes <= 8, sitdown/zero_sitdown <= 2).
    let cached = unsafe { MEMO[pos][princes][sitdown][zero_sitdown] };
    if cached != 0 {
        return cached;
    }

    let n = unsafe { N as usize };
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
            unsafe {
                RESULT[pos] = 1;
            }
            ret = calc_odd_1(pos + 1, princes - 1, 1, zero_sitdown);
        } else {
            ret = 0;
        }

        unsafe {
            RESULT[pos] = 0;
        }
        // Use wrapping add to emulate C unsigned int overflow behavior.
        ret = ret.wrapping_add(calc_odd_1(pos + 1, princes, 0, zero_sitdown));
    }

    unsafe {
        MEMO[pos][princes][sitdown][zero_sitdown] = ret;
    }
    ret
}

fn calc_odd() -> u32 {
    let mut ret: u32 = 0;

    unsafe {
        RESULT[0] = 1;
    }
    ret = ret.wrapping_add(calc_odd_1(1, N_OF_PRINCES - 1, 1, 1));

    unsafe {
        RESULT[0] = 0;
    }
    ret = ret.wrapping_add(calc_odd_1(1, N_OF_PRINCES, 0, 0));

    ret
}

fn calc_even_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize) -> u32 {
    let cached = unsafe { MEMO[pos][princes][sitdown][zero_sitdown] };
    if cached != 0 {
        return cached;
    }

    let n = unsafe { N as usize };
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
            unsafe {
                RESULT[pos] = 1;
                RESULT[pos + half] = 0;
            }
            ret = ret.wrapping_add(calc_even_1(pos + 1, princes - 1, 1, zero_sitdown));
        }
        if sitdown == 0 || sitdown == 1 {
            unsafe {
                RESULT[pos] = 0;
                RESULT[pos + half] = 1;
            }
            ret = ret.wrapping_add(calc_even_1(pos + 1, princes - 1, 2, zero_sitdown));
        }

        unsafe {
            RESULT[pos] = 0;
            RESULT[pos + half] = 0;
        }
        ret = ret.wrapping_add(calc_even_1(pos + 1, princes, 0, zero_sitdown));
    }

    unsafe {
        MEMO[pos][princes][sitdown][zero_sitdown] = ret;
    }
    ret
}

fn calc_even() -> u32 {
    let n = unsafe { N as usize };
    let half = n / 2;
    let mut ret: u32 = 0;

    unsafe {
        RESULT[0] = 1;
        RESULT[half] = 0;
    }
    ret = ret.wrapping_add(calc_even_1(1, N_OF_PRINCES - 1, 1, 1)); // zenhan

    unsafe {
        RESULT[0] = 0;
        RESULT[half] = 1;
    }
    ret = ret.wrapping_add(calc_even_1(1, N_OF_PRINCES - 1, 2, 2)); // kouhan

    unsafe {
        RESULT[0] = 0;
        RESULT[half] = 0;
    }
    ret = ret.wrapping_add(calc_even_1(1, N_OF_PRINCES, 0, 0)); // none

    ret
}

fn main() {
    // Read all input and parse as integers like repeated scanf("%d",&N).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    while let Some(tok) = it.next() {
        let n_val: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n_val == 0 {
            break;
        }

        unsafe {
            N = n_val;
            // memset(memo,0,sizeof(memo));
            MEMO = [[[[0u32; 3]; 3]; 9]; 70];
        }

        let ret: i64;
        if n_val <= 16 {
            ret = 0;
        } else if (n_val & 1) != 0 {
            ret = FACT8 * (calc_odd() as i64);
        } else {
            ret = FACT8 * (calc_even() as i64);
        }

        out.push_str(&format!("{ret}\n"));
    }

    print!("{out}");
}
