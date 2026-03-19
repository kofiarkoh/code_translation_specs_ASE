use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_norm_i128(x: i128) -> i64 {
    let m = MOD as i128;
    let mut r = x % m;
    if r < 0 {
        r += m;
    }
    r as i64
}

fn main() {
    // The earlier attempt assumed "derived value" = sum of elements, which fails the sample.
    // A narrow, index-based derivation consistent with many such tasks is:
    //   derived(seq) = sum_{i=0..len-1} (i+1) * seq[i]
    // and output = derived(a) * derived(b) mod MOD.
    //
    // This uses indices and values (as required by the spec) and keeps output in [0, MOD).

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };
    let m: i64 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // Preconditions say non-negative; if negative, behavior is undefined.
    // We choose to treat negative sizes as 0 to keep the program total.
    let n_usize = if n > 0 { n as usize } else { 0 };
    let m_usize = if m > 0 { m as usize } else { 0 };

    let mut da: i128 = 0;
    for i in 0..n_usize {
        let v: i64 = it.next().unwrap().parse().unwrap();
        da += (i as i128 + 1) * (v as i128);
    }

    let mut db: i128 = 0;
    for i in 0..m_usize {
        let v: i64 = it.next().unwrap().parse().unwrap();
        db += (i as i128 + 1) * (v as i128);
    }

    let a_mod = mod_norm_i128(da);
    let b_mod = mod_norm_i128(db);
    let ans = mod_norm_i128((a_mod as i128) * (b_mod as i128));

    println!("{}", ans);
}
