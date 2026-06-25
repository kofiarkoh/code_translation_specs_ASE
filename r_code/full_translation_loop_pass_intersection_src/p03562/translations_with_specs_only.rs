use std::io::{self, Read};

const N: usize = 4000;
const MOD: u64 = 998_244_353;

fn highest_set_bit(bits: &[u8; N]) -> Option<usize> {
    for i in (0..N).rev() {
        if bits[i] != 0 {
            return Some(i);
        }
    }
    None
}

fn is_zero(bits: &[u8; N]) -> bool {
    highest_set_bit(bits).is_none()
}

fn xor_shifted_into(dst: &mut [u8; N], src: &[u8; N], shift: usize) {
    // dst ^= src << shift, truncated to [0, N-1]
    if shift >= N {
        return;
    }
    for k in 0..(N - shift) {
        dst[k + shift] ^= src[k];
    }
}

fn parse_poly_from_bin_string(s: &str) -> [u8; N] {
    // Binary string length > N is undefined per spec; we do not guard.
    let bytes = s.as_bytes();
    let t = bytes.len();
    let mut p = [0u8; N];
    for k in 0..t {
        // coefficient of x^k is s[t-1-k]
        let ch = bytes[t - 1 - k];
        p[k] = (ch == b'1') as u8;
    }
    p
}

fn parse_bits_a(s: &str) -> Vec<u8> {
    // A bits stored as degrees 0..|A|-1, where degree k is s[|A|-1-k]
    let bytes = s.as_bytes();
    let t = bytes.len();
    let mut a = vec![0u8; t];
    for k in 0..t {
        a[k] = (bytes[t - 1 - k] == b'1') as u8;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let m: usize = it.next().unwrap().parse().unwrap();
    let a_str = it.next().unwrap();
    let a_bits = parse_bits_a(a_str);
    let a_len = a_bits.len();

    // Reduction phase: D starts as zero; A is not used here.
    let mut d = [0u8; N];

    for _ in 0..m {
        let b_str = it.next().unwrap();
        let mut b = parse_poly_from_bin_string(b_str);

        // Repeatedly reduce until B becomes zero.
        while !is_zero(&b) {
            let i = highest_set_bit(&d);
            let j = highest_set_bit(&b).unwrap();

            if i.is_none() {
                // If D is zero, swap roles so D has degree at least B.
                std::mem::swap(&mut d, &mut b);
                continue;
            }

            let i = i.unwrap();
            if i < j {
                std::mem::swap(&mut d, &mut b);
                continue;
            }

            let shift = i - j;
            xor_shifted_into(&mut d, &b, shift);
        }
    }

    // Determine L: one plus highest set bit of D, or 0 if D is zero.
    let l: i64 = match highest_set_bit(&d) {
        Some(h) => (h as i64) + 1,
        None => 0,
    };
    let l_minus_1: i64 = l - 1; // may be -1 when L=0

    // Precompute pow2 up to |A|-1 (bounded by N per spec model).
    let mut pow2 = vec![0u64; a_len.max(1)];
    pow2[0] = 1;
    for i in 1..a_len {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }

    // Counting phase.
    let mut r = [0u8; N];
    let mut ans: u64 = 0;
    let mut terminated = false;

    for i_usize in (0..a_len).rev() {
        let i = i_usize as i64;
        let x = a_bits[i_usize];
        let y = if i_usize < N { r[i_usize] } else { 0u8 };

        if i >= l_minus_1 && x == 1 {
            let exp = (i - l_minus_1) as usize;
            // exp is within [0, |A|-1] under the intended constraints.
            ans = (ans + pow2[exp]) % MOD;
        }

        if x != y {
            if i >= l_minus_1 {
                let shift = (i - l_minus_1) as usize;
                xor_shifted_into(&mut r, &d, shift);
                continue;
            } else {
                if x > y {
                    ans = (ans + 1) % MOD;
                }
                terminated = true;
                break;
            }
        }
    }

    if !terminated {
        ans = (ans + 1) % MOD;
    }

    println!("{}", ans % MOD);
}
