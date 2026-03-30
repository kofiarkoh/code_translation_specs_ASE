use std::io::{self, BufRead};

const FACT8: u64 = 40320;
const N_OF_PRINCES: usize = 8;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut memo = [[[0; 3]; 3]; 70]; // memoization array
    let mut result = [0; 70]; // result array

    for line in handle.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        // Reset memoization for each input
        for i in 0..70 {
            for j in 0..9 {
                for k in 0..3 {
                    memo[i][j][k] = 0;
                }
            }
        }

        let ret = if n <= 16 {
            0
        } else if n % 2 == 1 {
            (FACT8 * calc_odd(n, &mut result, &mut memo)) as u64
        } else {
            (FACT8 * calc_even(n, &mut result, &mut memo)) as u64
        };

        println!("{}", ret);
    }
}

fn calc_odd(n: usize, result: &mut [usize; 70], memo: &mut [[[u64; 3]; 3]; 70]) -> u64 {
    result[0] = 1;
    let mut ret = calc_odd_1(1, N_OF_PRINCES - 1, 1, 1, result, memo);
    result[0] = 0;
    ret += calc_odd_1(1, N_OF_PRINCES, 0, 0, result, memo);
    ret
}

fn calc_odd_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize, result: &mut [usize; 70], memo: &mut [[[u64; 3]; 3]; 70]) -> u64 {
    if memo[pos][princes][sitdown][zero_sitdown] != 0 {
        return memo[pos][princes][sitdown][zero_sitdown];
    }

    let ret = if princes == 0 {
        1
    } else if pos == N_OF_PRINCES - 1 && princes > 1 {
        0
    } else if pos == N_OF_PRINCES - 1 && princes == 1 {
        if zero_sitdown == 0 && sitdown == 0 {
            1
        } else {
            0
        }
    } else {
        let mut ret = 0;
        if sitdown == 0 {
            result[pos] = 1;
            ret += calc_odd_1(pos + 1, princes - 1, 1, zero_sitdown, result, memo);
        }
        result[pos] = 0;
        ret += calc_odd_1(pos + 1, princes, 0, zero_sitdown, result, memo);
        ret
    };

    memo[pos][princes][sitdown][zero_sitdown] = ret;
    ret
}

fn calc_even(n: usize, result: &mut [usize; 70], memo: &mut [[[u64; 3]; 3]; 70]) -> u64 {
    result[0] = 1;
    result[n / 2] = 0;
    let mut ret = calc_even_1(1, N_OF_PRINCES - 1, 1, 1, result, memo);
    result[0] = 0;
    result[n / 2] = 1;
    ret += calc_even_1(1, N_OF_PRINCES - 1, 2, 2, result, memo);
    result[0] = 0;
    result[n / 2] = 0;
    ret += calc_even_1(1, N_OF_PRINCES, 0, 0, result, memo);
    ret
}

fn calc_even_1(pos: usize, princes: usize, sitdown: usize, zero_sitdown: usize, result: &mut [usize; 70], memo: &mut [[[u64; 3]; 3]; 70]) -> u64 {
    if memo[pos][princes][sitdown][zero_sitdown] != 0 {
        return memo[pos][princes][sitdown][zero_sitdown];
    }

    let ret = if princes == 0 {
        1
    } else if pos == n / 2 - 1 && princes > 1 {
        0
    } else if pos == n / 2 - 1 && princes == 1 {
        if zero_sitdown == 0 && sitdown == 0 {
            2
        } else if (zero_sitdown == 2 || zero_sitdown == 0) && (sitdown == 1 || sitdown == 0) {
            1
        } else if (zero_sitdown == 1 || zero_sitdown == 0) && (sitdown == 2 || sitdown == 0) {
            1
        } else {
            0
        }
    } else {
        let mut ret = 0;
        if sitdown == 0 || sitdown == 2 {
            result[pos] = 1;
            result[pos + n / 2] = 0;
            ret += calc_even_1(pos + 1, princes - 1, 1, zero_sitdown, result, memo);
        }
        if sitdown == 0 || sitdown == 1 {
            result[pos] = 0;
            result[pos + n / 2] = 1;
            ret += calc_even_1(pos + 1, princes - 1, 2, zero_sitdown, result, memo);
        }
        result[pos] = 0;
        result[pos + n / 2] = 0;
        ret += calc_even_1(pos + 1, princes, 0, zero_sitdown, result, memo);
        ret
    };

    memo[pos][princes][sitdown][zero_sitdown] = ret;
    ret
}
