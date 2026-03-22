use std::io::{self, Read};

// Fixed capacity: number of base-1e9 limbs.
// If the true value exceeds this capacity, the produced value may be incorrect,
// matching the specification's "fixed-capacity big-integer storage" note.
const LIMBS: usize = 512;

fn main() {
    // Read three C `int` values via whitespace splitting.
    // If not exactly three values are read successfully, behavior is unspecified;
    // we choose to exit silently.
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let m: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let r: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    // C `int` overflow is UB per spec; Rust cannot model UB directly.
    // We emulate common 2's-complement behavior with wrapping arithmetic.
    let r_prime: i32 = r.wrapping_sub(n.wrapping_mul(m));

    if r_prime < 0 {
        println!("0");
        return;
    }
    if r_prime == 0 {
        println!("1");
        return;
    }

    // Spec says behavior is unspecified if n <= 0 while r' > 0.
    // Avoid inventing behavior: exit silently.
    if n <= 0 {
        return;
    }

    // Compute C(n + r' - 1, n - 1) using fixed-capacity big integer arithmetic.
    let n_minus_1: u32 = (n - 1) as u32;
    let rp_u32: u32 = r_prime as u32;
    let k: u32 = n_minus_1.min(rp_u32);

    // N = n + r' - 1 in mathematical integers; if this overflows, behavior is unspecified.
    // We compute in i128 and exit if negative.
    let n_total_i128: i128 = n as i128 + r_prime as i128 - 1;
    if n_total_i128 < 0 {
        return;
    }
    let n_total: u64 = n_total_i128 as u64;

    let mut num = BigFixed::one();
    let mut den = BigFixed::one();

    // Multiply numerator: product_{i=0..k-1} (N - k + 1 + i)
    let start = n_total
        .wrapping_sub(k as u64)
        .wrapping_add(1); // wrapping mirrors "unspecified" if it would underflow
    for i in 0..(k as u64) {
        num.mul_small(start.wrapping_add(i));
    }

    // Multiply denominator: k!
    for i in 1..=(k as u64) {
        den.mul_small(i);
    }

    // Exact division is expected; if not exact due to truncation/overflow, behavior is unspecified.
    let q = num.div_big(&den);
    println!("{}", q.to_decimal_string());
}

#[derive(Clone)]
struct BigFixed {
    // Little-endian base 1e9 limbs.
    d: [u32; LIMBS],
}

impl BigFixed {
    const BASE: u64 = 1_000_000_000;

    fn zero() -> Self {
        Self { d: [0u32; LIMBS] }
    }

    fn one() -> Self {
        let mut z = Self::zero();
        z.d[0] = 1;
        z
    }

    fn is_zero(&self) -> bool {
        self.d.iter().all(|&x| x == 0)
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in (0..LIMBS).rev() {
            let a = self.d[i];
            let b = other.d[i];
            if a != b {
                return a.cmp(&b);
            }
        }
        std::cmp::Ordering::Equal
    }

    fn sub_assign(&mut self, other: &Self) {
        // Assumes self >= other; if not, result wraps in base arithmetic (unspecified).
        let mut borrow: i64 = 0;
        for i in 0..LIMBS {
            let a = self.d[i] as i64 - borrow;
            let b = other.d[i] as i64;
            if a >= b {
                self.d[i] = (a - b) as u32;
                borrow = 0;
            } else {
                self.d[i] = (a + Self::BASE as i64 - b) as u32;
                borrow = 1;
            }
        }
    }

    fn mul_small(&mut self, m: u64) {
        // Multiply by a small u64, truncating overflow beyond LIMBS.
        let mut carry: u128 = 0;
        for i in 0..LIMBS {
            let v = self.d[i] as u128 * m as u128 + carry;
            self.d[i] = (v % Self::BASE as u128) as u32;
            carry = v / Self::BASE as u128;
        }
    }

    fn div_big(&self, divisor: &Self) -> Self {
        // Long division in base 1e9: compute quotient = self / divisor.
        // If divisor == 0, behavior is unspecified; we return 0.
        if divisor.is_zero() {
            return Self::zero();
        }
        if self.cmp(divisor) == std::cmp::Ordering::Less {
            return Self::zero();
        }

        // Highest non-zero limb indices.
        let mut d_hi = 0usize;
        for i in (0..LIMBS).rev() {
            if divisor.d[i] != 0 {
                d_hi = i;
                break;
            }
        }
        let mut n_hi = 0usize;
        for i in (0..LIMBS).rev() {
            if self.d[i] != 0 {
                n_hi = i;
                break;
            }
        }

        let mut q = Self::zero();
        let mut rem = self.clone();

        for shift in (0..=n_hi.saturating_sub(d_hi)).rev() {
            // Build shifted divisor.
            let mut ds = Self::zero();
            for i in 0..=d_hi {
                if i + shift < LIMBS {
                    ds.d[i + shift] = divisor.d[i];
                }
            }

            // Estimate quotient digit using top limbs.
            let r_top2 = {
                let hi = rem.d[d_hi + shift] as u128;
                let lo = if d_hi + shift >= 1 {
                    rem.d[d_hi + shift - 1] as u128
                } else {
                    0
                };
                hi * Self::BASE as u128 + lo
            };
            let d_top = divisor.d[d_hi] as u128;
            let mut qhat = if d_top == 0 {
                0u64
            } else {
                (r_top2 / d_top).min((Self::BASE - 1) as u128) as u64
            };

            if qhat == 0 {
                continue;
            }

            // Multiply ds by qhat and subtract; adjust qhat downward if needed.
            loop {
                let mut prod = ds.clone();
                prod.mul_small(qhat);

                if rem.cmp(&prod) != std::cmp::Ordering::Less {
                    rem.sub_assign(&prod);

                    // q += qhat * BASE^shift
                    let mut carry: u64 = qhat;
                    let mut idx = shift;
                    while carry != 0 && idx < LIMBS {
                        let v = q.d[idx] as u64 + carry;
                        q.d[idx] = (v % Self::BASE) as u32;
                        carry = v / Self::BASE;
                        idx += 1;
                    }
                    break;
                } else {
                    qhat -= 1;
                    if qhat == 0 {
                        break;
                    }
                }
            }
        }

        q
    }

    fn to_decimal_string(&self) -> String {
        // Convert base-1e9 limbs to decimal.
        let mut i = LIMBS;
        while i > 0 && self.d[i - 1] == 0 {
            i -= 1;
        }
        if i == 0 {
            return "0".to_string();
        }
        let mut s = String::new();
        s.push_str(&format!("{}", self.d[i - 1]));
        for j in (0..i - 1).rev() {
            s.push_str(&format!("{:09}", self.d[j]));
        }
        s
    }
}
