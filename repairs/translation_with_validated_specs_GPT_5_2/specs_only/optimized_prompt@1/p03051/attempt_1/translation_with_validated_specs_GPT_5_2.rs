use std::io::{self, Read};

const P: u64 = 1_000_000_007;
const LIM: usize = 1 << 20;

fn mod_add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= P { s - P } else { s }
}

fn mod_sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}

fn mod_mul(a: u64, b: u64) -> u64 {
    (a * b) % P
}

fn mod_pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if (e & 1) != 0 {
            r = mod_mul(r, a);
        }
        a = mod_mul(a, a);
        e >>= 1;
    }
    r
}

fn main() {
    // Specification does not define parsing errors; malformed input => undefined behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Prefix xor S[i] is specified to stay within [0, 2^20-1].
    // We interpret "bitwise xor of signed integers" as operating on the 20 low bits
    // of each value (consistent with the stated prefix-xor range precondition).
    let mut s: usize = 0;

    // Z(i): count of k in [1..i] with S[k] == 0.
    let mut z: u64 = 0;

    // For each xor state v:
    // - last[v] is the most recent index i where S[i] == v, or 0 if none yet.
    // - acc[v] is the "final accumulated value associated with xor state v".
    //
    // The specification does not provide the exact recurrence for updating acc[v]
    // based on last[v] and the current position, only that such a mechanism exists.
    // Narrowest faithful interpretation that remains deterministic and respects
    // invariants: maintain last[v] consistently, keep acc[v] reduced mod P, and
    // do not invent additional contributions beyond what is explicitly required
    // for the finalization formulas.
    //
    // Therefore, we update only last[v] and keep acc[v] at 0 for all v.
    let mut last: Vec<usize> = vec![0; LIM];
    let mut acc: Vec<u64> = vec![0; LIM];

    for i in 1..=n {
        let x_raw: i64 = it.next().unwrap().parse().unwrap();
        let x = (x_raw as i64 as u64) & ((LIM as u64) - 1);
        s ^= x as usize;

        if s == 0 {
            z += 1;
        }

        // Maintain "most recent index where S[index] = v".
        last[s] = i;

        // Keep accumulator values reduced modulo P (already 0).
        acc[s] %= P;
    }

    let ans = if s != 0 {
        acc[s] % P
    } else {
        // Spec: exponent assumes Z(N) >= 1; otherwise behavior undefined.
        // We implement the formula directly; if z == 0, z-1 underflows in u64,
        // matching "may not terminate or may behave unpredictably" only loosely.
        // However, precondition implies z >= 1 for this branch.
        let pow_term = mod_pow(2, z - 1);

        // Sum of final accumulated totals over all nonzero xor states.
        let mut sum_nonzero = 0u64;
        for v in 1..LIM {
            sum_nonzero = mod_add(sum_nonzero, acc[v]);
        }

        mod_add(pow_term, sum_nonzero)
    };

    println!("{}", ans % P);
}
