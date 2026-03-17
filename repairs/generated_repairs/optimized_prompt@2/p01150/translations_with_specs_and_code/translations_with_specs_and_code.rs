/*
  AOJ 2025
  Title: Eight Princes
  @kankichi573
*/

use std::io::{self, BufRead};

const FACT8: u64 = 40320;
const N_OF_PRINCES: usize = 8;

static mut MEMO: [[[[u64; 3]; 3]; 9]; 70] = [[[[0; 3]; 3]; 9]; 70];
static mut RESULT: [u8; 70] = [0; 70];

fn dump(f: usize, n: usize) {
    // Debugging function to print the current state of RESULT
    unsafe {
        match f {
            0 => {
                for i in 0..n {
                    print!("{}|", RESULT[i]);
                }
                println!();
            }
            1 => {
                for i in 0..n - 1 {
                    print!("{}|", RESULT[i]);
                }
                println!("1:");
            }
            2 => {
                for i in 0..n / 2 {
                    print!("{}|", RESULT[i]);
                }
                print!("1:");
                for i in n / 2..n {
                    print!("{}|", RESULT[i]);
                }
                println!("0:");
                for i in 0..n / 2 {
                    print!("{}|", RESULT[i]);
                }
                print!("0:");
                for i in n / 2..n {
                    print!("{}|", RESULT[i]);
                }
                println!("1+");
            }
            3 => {
                for i in 0..n / 2 {
                    print!("{}|", RESULT[i]);
                }
                print!("0:");
                for i in n / 2..n {
                    print!("{}|", RESULT[i]);
                }
                println!("1;");
            }
            4 => {
                for i in 0..n / 2 {
                    print!("{}|", RESULT[i]);
                }
                print!("1:");
                for i in n / 2..n {
                    print!("{}|", RESULT[i]);
                }
                println!("0@");
            }
            _ => {}
        }
    }
}

fn calc_odd_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize, n: usize) -> u64 {
    unsafe {
        if MEMO[pos][princes][sitdown][zero_sitdown] != 0 {
            return MEMO[pos][princes][sitdown][zero_sitdown];
        }

        let ret = if princes == 0 {
            dump(0, n);
            1
        } else if pos == n - 1 && princes > 1 {
            0
        } else if pos == n - 1 && princes == 1 {
            if zero_sitdown == 0 && sitdown == 0 {
                dump(1, n);
                1
            } else {
                0
            }
        } else {
            let mut ret = 0;
            if sitdown == 0 {
                RESULT[pos] = 1;
                ret += calc_odd_1(pos + 1, princes - 1, 1, zero_sitdown, n);
            }
            RESULT[pos] = 0;
            ret += calc_odd_1(pos + 1, princes, 0, zero_sitdown, n);
            ret
        };

        MEMO[pos][princes][sitdown][zero_sitdown] = ret;
        ret
    }
}

fn calc_odd(n: usize) -> u64 {
    unsafe {
        RESULT[0] = 1;
        let mut ret = calc_odd_1(1, N_OF_PRINCES - 1, 1, 1, n);
        RESULT[0] = 0;
        ret += calc_odd_1(1, N_OF_PRINCES, 0, 0, n);
        ret
    }
}

fn calc_even_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize, n: usize) -> u64 {
    unsafe {
        if MEMO[pos][princes][sitdown][zero_sitdown] != 0 {
            return MEMO[pos][princes][sitdown][zero_sitdown];
        }

        let ret = if princes == 0 {
            dump(0, n);
            1
        } else if pos == n / 2 - 1 && princes > 1 {
            0
        } else if pos == n / 2 - 1 && princes == 1 {
            if zero_sitdown == 0 && sitdown == 0 {
                dump(2, n);
                2
            } else if (zero_sitdown == 2 || zero_sitdown == 0) && (sitdown == 1 || sitdown == 0) {
                dump(3, n);
                1
            } else if (zero_sitdown == 1 || zero_sitdown == 0) && (sitdown == 2 || sitdown == 0) {
                dump(4, n);
                1
            } else {
                0
            }
        } else {
            let mut ret = 0;
            if sitdown == 0 || sitdown == 2 {
                RESULT[pos] = 1;
                RESULT[pos + n / 2] = 0;
                ret += calc_even_1(pos + 1, princes - 1, 1, zero_sitdown, n);
            }
            if sitdown == 0 || sitdown == 1 {
                RESULT[pos] = 0;
                RESULT[pos + n / 2] = 1;
                ret += calc_even_1(pos + 1, princes - 1, 2, zero_sitdown, n);
            }

            RESULT[pos] = 0;
            RESULT[pos + n / 2] = 0;
            ret += calc_even_1(pos + 1, princes, 0, zero_sitdown, n);
            ret
        };

        MEMO[pos][princes][sitdown][zero_sitdown] = ret;
        ret
    }
}

fn calc_even(n: usize) -> u64 {
    unsafe {
        RESULT[0] = 1;
        RESULT[n / 2] = 0;
        let mut ret = calc_even_1(1, N_OF_PRINCES - 1, 1, 1, n); // zenhan
        RESULT[0] = 0;
        RESULT[n / 2] = 1;
        ret += calc_even_1(1, N_OF_PRINCES - 1, 2, 2, n); // kouhan
        RESULT[0] = 0;
        RESULT[n / 2] = 0;
        ret += calc_even_1(1, N_OF_PRINCES, 0, 0, n); // none
        ret
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        unsafe {
            // Reset memoization array
            for i in 0..70 {
                for j in 0..9 {
                    for k in 0..3 {
                        for l in 0..3 {
                            MEMO[i][j][k][l] = 0;
                        }
                    }
                }
            }
        }

        let ret = if n <= 16 {
            0
        } else if n % 2 == 1 {
            (FACT8 * calc_odd(n)) as u64
        } else {
            (FACT8 * calc_even(n)) as u64
        };

        println!("{}", ret);
    }
}
