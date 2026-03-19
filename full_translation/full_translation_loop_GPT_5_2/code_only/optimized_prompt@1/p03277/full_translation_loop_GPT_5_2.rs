use std::io::{self, Read};

type Int64 = i64;

fn init_bit(bit: &mut [i32]) {
    let n = bit[0] as usize;
    for i in 1..=n {
        bit[i] = 0;
    }
}

fn inc(bit: &mut [i32], x: usize) {
    let n = bit[0] as usize;
    let mut i = x;
    while i <= n {
        bit[i] = bit[i].wrapping_add(1); // match C int overflow behavior (wrap)
        let lsb = i & (!i + 1); // i & -i, but in usize
        i += lsb;
    }
}

fn get_sum(bit: &[i32], x: usize) -> i32 {
    let mut res: i32 = 0;
    let mut i = x;
    while i > 0 {
        res = res.wrapping_add(bit[i]); // match C int overflow behavior (wrap)
        let lsb = i & (!i + 1);
        i -= lsb;
    }
    res
}

fn count(m: i32, a: &[i32], n: usize, bit: &mut Vec<i32>) -> Int64 {
    // C uses a static bit buffer allocated once sized by first n.
    // Here we allocate once in run() with the same size for the given n.
    init_bit(bit);

    let mut cnt: Int64 = 0;
    let mut sum: i32 = (n as i32) + 1;
    inc(bit, sum as usize);

    for &ai in a.iter().take(n) {
        sum = sum.wrapping_add(if ai < m { -1 } else { 1 });
        cnt = cnt.wrapping_add(get_sum(bit, sum as usize) as Int64);
        inc(bit, sum as usize);
    }
    cnt
}

fn main() {
    // Read all input, parse as ints like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => return,
    };

    let mut a = vec![0i32; n];
    for i in 0..n {
        if let Some(s) = it.next() {
            a[i] = s.parse::<i32>().unwrap();
        }
    }

    // bit size: 2*n+1+1, with bit[0]=2*n+1 (same as C)
    let bit_n = 2 * n + 1;
    let mut bit = vec![0i32; bit_n + 1];
    bit[0] = bit_n as i32;

    let mut l: i32 = 1;
    let mut r: i32 = 1_000_000_000i32 + 1;

    while (r - l) > 1 {
        // In C, (l+r)/2 is int arithmetic; here it won't overflow for these bounds.
        let m = (l + r) / 2;
        let cnt = count(m, &a, n, &mut bit);
        let left = 4i64 * cnt;
        let right = (n as i64) * ((n as i64) + 1);
        if left >= right {
            l = m;
        } else {
            r = m;
        }
    }

    print!("{l}\n");
}
