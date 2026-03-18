use std::io::{self, Read};

const TABAICHO_SIZE: usize = 8667;
const TABAICHO_BLOCK: i64 = 1_000_000_000;

static mut TABAICHO_MAIN: [i32; TABAICHO_SIZE] = [0; TABAICHO_SIZE];

fn tabaicho_set(num: i32) {
    // Uses a global array like the C code; safe wrapper around the same semantics.
    unsafe {
        TABAICHO_MAIN[0] = num;
        for i in 1..TABAICHO_SIZE {
            TABAICHO_MAIN[i] = 0;
        }
    }
}

fn tabaicho_mul(num: i32) {
    let mut carry: i64 = 0;
    unsafe {
        for i in 0..TABAICHO_SIZE {
            let mut calctemp: i64 = (TABAICHO_MAIN[i] as i64) * (num as i64);
            calctemp += carry;
            TABAICHO_MAIN[i] = (calctemp % TABAICHO_BLOCK) as i32;
            carry = calctemp / TABAICHO_BLOCK;
        }
    }
}

fn tabaicho_div(num: i32) {
    let mut remainder: i64 = 0;
    let num64 = num as i64;
    unsafe {
        for i in (0..TABAICHO_SIZE).rev() {
            remainder = remainder * TABAICHO_BLOCK + (TABAICHO_MAIN[i] as i64);
            TABAICHO_MAIN[i] = (remainder / num64) as i32;
            remainder %= num64;
        }
    }
}

fn tabaicho_print() {
    unsafe {
        // Match C behavior: if all digits are zero, the C code would access out of bounds (UB).
        // In this program's control flow, print is only called when the value is positive.
        let mut i: isize = (TABAICHO_SIZE as isize) - 1;
        while i >= 0 {
            if TABAICHO_MAIN[i as usize] > 0 {
                break;
            }
            i -= 1;
        }

        // Assume i >= 0 as per intended usage.
        let mut out = String::new();
        out.push_str(&format!("{}", TABAICHO_MAIN[i as usize]));
        i -= 1;
        while i >= 0 {
            out.push_str(&format!("{:09}", TABAICHO_MAIN[i as usize]));
            i -= 1;
        }
        out.push('\n');
        print!("{out}");
    }
}

fn main() {
    // Read all input, parse like scanf("%d%d%d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let m: i32 = it.next().unwrap().parse().unwrap();
    let mut r: i32 = it.next().unwrap().parse().unwrap();

    // r -= n*m; (C int arithmetic; assume no overflow for typical inputs)
    r = r - n * m;

    if r < 0 {
        println!("0");
    } else if r == 0 {
        println!("1");
    } else {
        // Compute (n+r-1)C(n-1) with big integer in base 1e9 blocks.
        tabaicho_set(1);

        let mut n2: i32 = n - 1;
        let a: i32 = n + r - 1;
        let b: i32 = n - 1;
        if a - b < b {
            n2 = a - b;
        }

        let mut j: i32 = n + r - 1;
        for _i in 0..n2 {
            tabaicho_mul(j);
            j -= 1;
        }
        for i in 1..=n2 {
            tabaicho_div(i);
        }
        tabaicho_print();
    }
}
