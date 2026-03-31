use std::io::{self, Read};

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    // Read entire stdin and parse three C-like `int` values.
    // If fewer than three are present, behavior is unspecified by the spec;
    // we simply return without output.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };
    let m: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };
    let r: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };

    // Compute r' = r - n*m in C `int` arithmetic.
    // The spec states overflow is undefined in C; in Rust we choose wrapping
    // to emulate a concrete (but not guaranteed) machine-like behavior without panicking.
    let rp: i32 = r.wrapping_sub(n.wrapping_mul(m));

    if rp < 0 {
        println!("0");
        return;
    }
    if rp == 0 {
        println!("1");
        return;
    }

    // For rp > 0, compute C(n + rp - 1, n - 1) exactly via big-integer arithmetic.
    // The spec mentions a fixed-capacity big integer; here we use an unbounded BigUint.
    // If n <= 0 in this branch, behavior is unspecified; we do not add extra handling.
    let n_i64 = n as i64;
    let rp_i64 = rp as i64;

    let a = n_i64 + rp_i64 - 1; // top of binomial
    let b1 = n_i64 - 1;
    let b2 = rp_i64;
    let k = if b1 < b2 { b1 } else { b2 }; // smaller of (n-1) and rp

    // Minimal BigUint implementation in base 1e9.
    #[derive(Clone)]
    struct BigUint {
        // little-endian limbs, base 1e9
        d: Vec<u32>,
    }
    impl BigUint {
        const BASE: u64 = 1_000_000_000;

        fn one() -> Self {
            Self { d: vec![1] }
        }

        fn normalize(&mut self) {
            while self.d.len() > 1 && *self.d.last().unwrap() == 0 {
                self.d.pop();
            }
        }

        fn mul_u128(&mut self, m: u128) {
            if m == 0 {
                self.d.clear();
                self.d.push(0);
                return;
            }
            let mut carry: u128 = 0;
            for limb in &mut self.d {
                let cur = (*limb as u128) * m + carry;
                *limb = (cur % (Self::BASE as u128)) as u32;
                carry = cur / (Self::BASE as u128);
            }
            while carry != 0 {
                let limb = (carry % (Self::BASE as u128)) as u32;
                self.d.push(limb);
                carry /= Self::BASE as u128;
            }
            self.normalize();
        }

        fn div_u128_exact(&mut self, div: u128) {
            // Exact division by small integer; assumes remainder becomes 0.
            let mut rem: u128 = 0;
            for i in (0..self.d.len()).rev() {
                let cur = rem * (Self::BASE as u128) + (self.d[i] as u128);
                self.d[i] = (cur / div) as u32;
                rem = cur % div;
            }
            // If not exact, spec says behavior is unspecified; we keep the truncated quotient.
            self.normalize();
        }

        fn to_string(&self) -> String {
            let mut s = String::new();
            if self.d.is_empty() {
                return "0".to_string();
            }
            let mut it = self.d.iter().rev();
            if let Some(&first) = it.next() {
                s.push_str(&first.to_string());
            }
            for &limb in it {
                s.push_str(&format!("{:09}", limb));
            }
            s
        }
    }

    // Compute binomial using multiplicative formula with gcd cancellations to keep intermediates small:
    // result = product_{i=1..k} (a - k + i) / i
    // We maintain a list of denominators and cancel gcds against numerators before multiplying.
    let k_usize = if k <= 0 { 0 } else { k as usize };
    let mut den: Vec<u128> = (1..=k_usize).map(|x| x as u128).collect();

    let start = a - (k as i64) + 1;
    let mut res = BigUint::one();

    for t in 0..k_usize {
        let mut num: u128 = (start + t as i64) as u128;

        // Cancel numerator with remaining denominators.
        for d in den.iter_mut() {
            if *d == 1 {
                continue;
            }
            let g = gcd_u128(num, *d);
            if g > 1 {
                num /= g;
                *d /= g;
                if num == 1 {
                    break;
                }
            }
        }

        res.mul_u128(num);
    }

    // Divide by any remaining denominators (should all be 1 if cancellations were complete).
    for d in den {
        if d != 1 {
            res.div_u128_exact(d);
        }
    }

    println!("{}", res.to_string());
}
