use std::io::{self, Read};

const FACT8: i64 = 40320;
const N_OF_PRINCES: usize = 8;

// Memo table: memo[pos][princes][sitdown][zero_sitdown] -> u32
// 0 means "not computed yet" (matches C behavior; computed values are always > 0).
type Memo = Vec<Vec<Vec<Vec<u32>>>>;

fn calc_odd_1(
    n: usize,
    memo: &mut Memo,
    pos: usize,
    princes: usize,
    sitdown: usize,
    zero_sitdown: usize,
) -> u32 {
    let cached = memo[pos][princes][sitdown][zero_sitdown];
    if cached != 0 {
        return cached;
    }

    let ret: u32;
    if princes == 0 {
        ret = 1;
    } else if pos == n - 1 && princes > 1 {
        ret = 0;
    } else if pos == n - 1 && princes == 1 {
        if zero_sitdown == 0 && sitdown == 0 {
            ret = 1;
        } else {
            ret = 0;
        }
    } else {
        let mut sum = 0u32;

        // Place a prince at pos (represented as 1) only if previous was empty.
        if sitdown == 0 {
            sum += calc_odd_1(n, memo, pos + 1, princes - 1, 1, zero_sitdown);
        }

        // Leave empty at pos (represented as 0).
        sum += calc_odd_1(n, memo, pos + 1, princes, 0, zero_sitdown);

        ret = sum;
    }

    memo[pos][princes][sitdown][zero_sitdown] = ret;
    ret
}

fn calc_odd(n: usize, memo: &mut Memo) -> u32 {
    // Mirrors C: two initial conditions for position 0.
    let mut ret = 0u32;

    // result[0]=1; ret += calc_odd_1(1, 7, 1, 1);
    ret += calc_odd_1(n, memo, 1, N_OF_PRINCES - 1, 1, 1);

    // result[0]=0; ret += calc_odd_1(1, 8, 0, 0);
    ret += calc_odd_1(n, memo, 1, N_OF_PRINCES, 0, 0);

    ret
}

fn calc_even_1(
    n: usize,
    memo: &mut Memo,
    pos: usize,
    princes: usize,
    sitdown: usize,
    zero_sitdown: usize,
) -> u32 {
    let cached = memo[pos][princes][sitdown][zero_sitdown];
    if cached != 0 {
        return cached;
    }

    let half = n / 2;
    let ret: u32;

    if princes == 0 {
        ret = 1;
    } else if pos == half - 1 && princes > 1 {
        ret = 0;
    } else if pos == half - 1 && princes == 1 {
        // Base-case logic exactly as in C.
        if zero_sitdown == 0 && sitdown == 0 {
            ret = 2;
        } else if (zero_sitdown == 2 || zero_sitdown == 0) && (sitdown == 1 || sitdown == 0) {
            ret = 1;
        } else if (zero_sitdown == 1 || zero_sitdown == 0) && (sitdown == 2 || sitdown == 0) {
            ret = 1;
        } else {
            ret = 0;
        }
    } else {
        let mut sum = 0u32;

        // Place (1,0) across the two halves if allowed by sitdown state.
        if sitdown == 0 || sitdown == 2 {
            sum += calc_even_1(n, memo, pos + 1, princes - 1, 1, zero_sitdown);
        }
        // Place (0,1) across the two halves if allowed by sitdown state.
        if sitdown == 0 || sitdown == 1 {
            sum += calc_even_1(n, memo, pos + 1, princes - 1, 2, zero_sitdown);
        }

        // Place (0,0) (no prince in either half at this position).
        sum += calc_even_1(n, memo, pos + 1, princes, 0, zero_sitdown);

        ret = sum;
    }

    memo[pos][princes][sitdown][zero_sitdown] = ret;
    ret
}

fn calc_even(n: usize, memo: &mut Memo) -> u32 {
    let mut ret = 0u32;

    // result[0]=1; result[n/2]=0; ret += calc_even_1(1,7,1,1);
    ret += calc_even_1(n, memo, 1, N_OF_PRINCES - 1, 1, 1);

    // result[0]=0; result[n/2]=1; ret += calc_even_1(1,7,2,2);
    ret += calc_even_1(n, memo, 1, N_OF_PRINCES - 1, 2, 2);

    // result[0]=0; result[n/2]=0; ret += calc_even_1(1,8,0,0);
    ret += calc_even_1(n, memo, 1, N_OF_PRINCES, 0, 0);

    ret
}

fn make_memo() -> Memo {
    // C sizes: memo[70][9][3][3]
    // We'll allocate exactly those bounds.
    let pos = 70usize;
    let princes = 9usize;
    let sitdown = 3usize;
    let zero = 3usize;
    vec![vec![vec![vec![0u32; zero]; sitdown]; princes]; pos]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();
    for tok in input.split_whitespace() {
        let n: i32 = tok.parse().unwrap();
        if n == 0 {
            break;
        }

        // SPEC says: "If N is less than or equal to 16, the output will be zero."
        // This matches the C code (even though it contradicts the earlier precondition text).
        let ans: i64 = if n <= 16 {
            0
        } else {
            let n_usize = n as usize;
            let mut memo = make_memo();
            if (n & 1) != 0 {
                FACT8 * (calc_odd(n_usize, &mut memo) as i64)
            } else {
                FACT8 * (calc_even(n_usize, &mut memo) as i64)
            }
        };

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
