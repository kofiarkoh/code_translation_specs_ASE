use std::io::{self, Read};

const TABAICHO_SIZE: usize = 8667;
const TABAICHO_BLOCK: i64 = 1_000_000_000; // 9 digits per block

fn tabaicho_set(a: &mut [i32; TABAICHO_SIZE], num: i32) {
    a[0] = num;
    for i in 1..TABAICHO_SIZE {
        a[i] = 0;
    }
}

fn tabaicho_mul(a: &mut [i32; TABAICHO_SIZE], num: i32) {
    let mut carry: i64 = 0;
    let num64 = num as i64;
    for i in 0..TABAICHO_SIZE {
        let mut tmp: i64 = (a[i] as i64) * num64;
        tmp += carry;
        a[i] = (tmp % TABAICHO_BLOCK) as i32;
        carry = tmp / TABAICHO_BLOCK;
    }
}

fn tabaicho_div(a: &mut [i32; TABAICHO_SIZE], num: i32) {
    let mut rem: i64 = 0;
    let num64 = num as i64;
    for i in (0..TABAICHO_SIZE).rev() {
        rem = rem * TABAICHO_BLOCK + (a[i] as i64);
        a[i] = (rem / num64) as i32;
        rem %= num64;
    }
}

fn tabaicho_print(a: &[i32; TABAICHO_SIZE]) {
    // Match C behavior: find highest non-zero block, then print it without padding,
    // then print remaining blocks with %09d, then newline.
    let mut i: isize = (TABAICHO_SIZE as isize) - 1;
    while i >= 0 {
        if a[i as usize] > 0 {
            break;
        }
        i -= 1;
    }

    // In the original C, if all blocks are zero, this would access out of bounds (UB).
    // For the given program flow, the value printed is never zero here (r>0 => combination > 0).
    // We still guard to avoid panic while keeping reasonable behavior.
    if i < 0 {
        println!("0");
        return;
    }

    print!("{}", a[i as usize]);
    i -= 1;
    while i >= 0 {
        print!("{:09}", a[i as usize]);
        i -= 1;
    }
    println!();
}

fn main() {
    // Read all input, parse three ints like scanf("%d%d%d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
    let m: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
    let mut r: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);

    // r -= n*m; (C int overflow is UB; we use wrapping to emulate typical 2's complement behavior)
    r = r.wrapping_sub(n.wrapping_mul(m));

    if r < 0 {
        print!("0\n");
    } else if r == 0 {
        print!("1\n");
    } else {
        // Compute (n+r-1)C(n-1) using big integer in base 1e9 blocks.
        let mut tabaicho_main = [0i32; TABAICHO_SIZE];
        tabaicho_set(&mut tabaicho_main, 1);

        let mut n2: i32 = n - 1;
        // if ((n+r-1)-(n-1) < n-1) n2 = (n+r-1)-(n-1);
        // Note: (n+r-1)-(n-1) == r, but keep structure.
        let left = (n.wrapping_add(r).wrapping_sub(1)).wrapping_sub(n.wrapping_sub(1));
        if left < n.wrapping_sub(1) {
            n2 = left;
        }

        // for(i=0,j=n+r-1;i<n2;i++,j--) mul(j);
        let mut j: i32 = n.wrapping_add(r).wrapping_sub(1);
        let mut i: i32 = 0;
        while i < n2 {
            tabaicho_mul(&mut tabaicho_main, j);
            i += 1;
            j = j.wrapping_sub(1);
        }

        // for(i=1;i<=n2;i++) div(i);
        let mut d: i32 = 1;
        while d <= n2 {
            tabaicho_div(&mut tabaicho_main, d);
            d += 1;
        }

        tabaicho_print(&tabaicho_main);
    }
}
