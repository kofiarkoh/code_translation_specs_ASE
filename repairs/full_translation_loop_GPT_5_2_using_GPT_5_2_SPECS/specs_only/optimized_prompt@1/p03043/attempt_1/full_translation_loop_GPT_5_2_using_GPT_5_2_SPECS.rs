use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Memory proportional to N, as required by the specification.
    let mut s: Vec<i64> = vec![0; n as usize];

    for (idx, sj) in s.iter_mut().enumerate() {
        let j = (idx as i64) + 1;
        let mut p: i64 = 1;

        // Find the minimal power of two p such that k <= p * j.
        // The specification assumes this terminates and does not overflow.
        while k > p * j {
            p *= 2;
        }
        *sj = p;
    }

    let mut ans: f64 = 0.0;
    for &p in &s {
        ans += 1.0f64 / (p as f64);
    }

    print!("{:.12}", ans);
}
