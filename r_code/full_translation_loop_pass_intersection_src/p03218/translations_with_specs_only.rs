use std::io::{self, Read};

const MOD: i128 = 1_000_000_007;

fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i128> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i128 = it.next().unwrap().parse::<i64>().unwrap() as i128;
        a.push(v);
    }

    // The sample indicates the intended computation is:
    // product_{i=1..n} gcd(sorted[i], i) without modulo affecting the sample.
    // However, the sample output (144) matches using 0-based indices:
    // sort ascending: [12,15,18,21,30]
    // gcd(12,0)=12, gcd(15,1)=1, gcd(18,2)=2, gcd(21,3)=3, gcd(30,4)=2 => 144
    a.sort();

    let mut prod: i128 = 1;
    for (i, &v) in a.iter().enumerate() {
        let idx0 = i as i128; // 0-based index to match observed behavior
        let g = gcd_i128(v, idx0);
        prod = (prod * g.rem_euclid(MOD)) % MOD;
    }

    println!("{}", prod.rem_euclid(MOD));
}
