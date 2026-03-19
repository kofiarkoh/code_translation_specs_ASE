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
        bit[i] += 1;
        // i += i & -i  (Fenwick lowbit)
        let lowbit = i & (!i + 1);
        i += lowbit;
    }
}

fn get_sum(bit: &[i32], x: usize) -> i32 {
    let mut res: i32 = 0;
    let mut i = x;
    while i > 0 {
        res += bit[i];
        let lowbit = i & (!i + 1);
        i -= lowbit;
    }
    res
}

fn count(m: i32, a: &[i32], bit: &mut Vec<i32>) -> Int64 {
    // C uses a static BIT allocated once sized by the first n.
    // Here we allocate/resize per call to match current n (safe and semantically equivalent
    // for this program, which calls count with a fixed n).
    let n = a.len();
    let size = 2 * n + 1;
    if bit.len() != size + 1 {
        bit.clear();
        bit.resize(size + 1, 0);
        bit[0] = size as i32;
    } else {
        bit[0] = size as i32;
    }

    init_bit(bit);

    let mut cnt: Int64 = 0;
    let mut sum: i32 = n as i32 + 1;
    inc(bit, sum as usize);

    for &ai in a {
        sum += if ai < m { -1 } else { 1 };
        cnt += get_sum(bit, sum as usize) as Int64;
        inc(bit, sum as usize);
    }
    cnt
}

fn main() {
    // Read all input (scanf-style whitespace separated integers)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut a = vec![0i32; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    let mut l: i32 = 1;
    let mut r: i32 = 1_000_000_000 + 1;

    let mut bit: Vec<i32> = Vec::new();

    while (r - l) > 1 {
        let m = (l + r) / 2;
        let cnt = count(m, &a, &mut bit);

        // Preserve C's integer behavior: use i64 for products to avoid overflow.
        if 4i64 * cnt >= (n as i64) * ((n as i64) + 1) {
            l = m;
        } else {
            r = m;
        }
    }

    print!("{l}\n");
}
