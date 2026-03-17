/*
  AOJ 2025
  Title: Eight Princes
  @kankichi573
*/

use std::io::{self, BufRead};
use std::mem;

const FACT8: u64 = 40320;
const N_OF_PRINCES: usize = 8;

static mut N: usize = 0;
static mut RESULT: [u8; 70] = [0; 70];

#[cfg(debug_assertions)]
fn dump(f: usize) {
    unsafe {
        match f {
            0 => {
                for i in 0..N {
                    print!("{}|", RESULT[i]);
                }
                println!();
            }
            1 => {
                for i in 0..N - 1 {
                    print!("{}|", RESULT[i]);
                }
                println!("1:");
            }
            2 => {
                for i in 0..N / 2 - 1 {
                    print!("{}|", RESULT[i]);
                }
                print!("1:");
                for i in N / 2..N - 1 {
                    print!("{}|", RESULT[i]);
                }
                println!("0:");
                for i in 0..N / 2 - 1 {
                    print!("{}|", RESULT[i]);
                }
                print!("0:");
                for i in N / 2..N - 1 {
                    print!("{}|", RESULT[i]);
                }
                println!("1+");
            }
            3 => {
                for i in 0..N / 2 - 1 {
                    print!("{}|", RESULT[i]);
                }
                print!("0:");
                for i in N / 2..N - 1 {
                    print!("{}|", RESULT[i]);
                }
                println!("1;");
            }
            4 => {
                for i in 0..N / 2 - 1 {
                    print!("{}|", RESULT[i]);
                }
                print!("1:");
                for i in N / 2..N - 1 {
                    print!("{}|", RESULT[i]);
                }
                println!("0@");
            }
            _ => {}
        }
    }
}

static mut MEMO: [[[[u64; 3]; 3]; 70]; 70] = [[[[0; 3]; 3]; 70]; 70];

unsafe fn calc_odd_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize) -> u64 {
    let mut ret: u64;

    if MEMO[pos][princes][sitdown][zero_sitdown] != 0 {
        return MEMO[pos][princes][sitdown][zero_sitdown];
    }

    if princes == 0 {
        dump(0);
        ret = 1;
    } else if pos == N - 1 && princes > 1 {
        ret = 0;
    } else if pos == N - 1 && princes == 1 {
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
        ret += calc_odd_1(pos + 1, princes, 0, zero_sitdown);
    }
    MEMO[pos][princes][sitdown][zero_sitdown] = ret;
    ret
}

unsafe fn calc_odd(n: usize) -> u64 {
    let mut ret = 0;

    RESULT[0] = 1;
    ret += calc_odd_1(1, N_OF_PRINCES - 1, 1, 1);
    RESULT[0] = 0;
    ret += calc_odd_1(1, N_OF_PRINCES, 0, 0);
    ret
}

unsafe fn calc_even_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize) -> u64 {
    let mut ret: u64;

    if MEMO[pos][princes][sitdown][zero_sitdown] != 0 {
        return MEMO[pos][princes][sitdown][zero_sitdown];
    }

    if princes == 0 {
        dump(0);
        ret = 1;
    } else if pos == N / 2 - 1 && princes > 1 {
        ret = 0;
    } else if pos == N / 2 - 1 && princes == 1 {
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
            RESULT[pos + N / 2] = 0;
            ret += calc_even_1(pos + 1, princes - 1, 1, zero_sitdown);
        }
        if sitdown == 0 || sitdown == 1 {
            RESULT[pos] = 0;
            RESULT[pos + N / 2] = 1;
            ret += calc_even_1(pos + 1, princes - 1, 2, zero_sitdown);
        }

        RESULT[pos] = 0;
        RESULT[pos + N / 2] = 0;
        ret += calc_even_1(pos + 1, princes, 0, zero_sitdown);
    }
    MEMO[pos][princes][sitdown][zero_sitdown] = ret;
    ret
}

unsafe fn calc_even(n: usize) -> u64 {
    let mut ret = 0;

    RESULT[0] = 1;
    RESULT[n / 2] = 0;
    ret += calc_even_1(1, N_OF_PRINCES - 1, 1, 1); // zenhan
    RESULT[0] = 0;
    RESULT[n / 2] = 1;
    ret += calc_even_1(1, N_OF_PRINCES - 1, 2, 2); // kouhan
    RESULT[0] = 0;
    RESULT[n / 2] = 0;
    ret += calc_even_1(1, N_OF_PRINCES, 0, 0); // none
    ret
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        unsafe {
            N = n;
            MEMO = [[[[0; 3]; 3]; 70]; 70]; // Clear memo
            if N <= 16 {
                println!("0");
            } else if N % 2 == 1 {
                let ret = (FACT8 as u64) * calc_odd(N);
                println!("{}", ret);
            } else {
                let ret = (FACT8 as u64) * calc_even(N);
                println!("{}", ret);
            }
        }
    }
}
