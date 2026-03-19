use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read};

fn is_perfect_square_u64(x: u64) -> bool {
    let r = (x as f64).sqrt() as u64;
    r * r == x || (r + 1) * (r + 1) == x
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // Sieve primes up to 2221 (as implied by the specification)
    const SIEVE_MAX: usize = 2221;
    let mut is_comp = vec![false; SIEVE_MAX + 1];
    let mut primes: Vec<u64> = Vec::new();
    for i in 2..=SIEVE_MAX {
        if !is_comp[i] {
            primes.push(i as u64);
            let mut j = i * i;
            while j <= SIEVE_MAX {
                is_comp[j] = true;
                j += i;
            }
        }
    }

    // Group counts for reduced values (processable only, excluding v=1 special handling later)
    let mut counts: BTreeMap<u64, u64> = BTreeMap::new();
    let mut result: u64 = 0;

    for _ in 0..n {
        let x0: i64 = it.next().unwrap().parse().unwrap();
        // Preconditions say non-negative and meaningful values >= 1.
        let mut x: u64 = x0 as u64;

        // Factor by small primes; track exponents mod 3 and build cube-free reduction.
        let mut reduced: u64 = 1;

        for &p in &primes {
            let pp = p * p;
            if pp > x {
                break;
            }
            if x % p == 0 {
                let mut e: u32 = 0;
                while x % p == 0 {
                    x /= p;
                    e += 1;
                }
                let r = e % 3;
                if r == 1 {
                    reduced = reduced.saturating_mul(p);
                } else if r == 2 {
                    reduced = reduced.saturating_mul(p.saturating_mul(p));
                }
            }
        }

        // Residual handling rule:
        // If remaining factor > 100000 and not a perfect square => unprocessable-large-residual.
        if x > 100_000 && !is_perfect_square_u64(x) {
            result += 1;
            continue;
        }

        // Otherwise incorporate residual into reduced value.
        // If x is a perfect square, treat it as having exponent 2 (mod 3) for its base.
        if x > 1 {
            if is_perfect_square_u64(x) {
                let r = (x as f64).sqrt() as u64;
                // r is the base; exponent is 2 => multiply by r^2 (i.e., x itself).
                reduced = reduced.saturating_mul(r.saturating_mul(r));
            } else {
                // residual is treated as exponent 1
                reduced = reduced.saturating_mul(x);
            }
        }

        *counts.entry(reduced).or_insert(0) += 1;
    }

    // Special case: v = 1 contributes exactly 1 (if present at all), regardless of multiplicity.
    if counts.remove(&1).is_some() {
        result += 1;
    }

    // Pairing logic: for each remaining v, compute complement c(v) such that v*c(v) is a cube
    // under the same factorization limits. If c(v) exceeds 1e10, treat as unpairable.
    const COMP_LIMIT: u64 = 10_000_000_000;

    let mut processed: BTreeSet<u64> = BTreeSet::new();
    let keys: Vec<u64> = counts.keys().copied().collect();

    for v in keys {
        if processed.contains(&v) {
            continue;
        }
        let cv_opt = {
            let mut t = v;
            let mut comp: u64 = 1;

            for &p in &primes {
                let pp = p * p;
                if pp > t {
                    break;
                }
                if t % p == 0 {
                    let mut e: u32 = 0;
                    while t % p == 0 {
                        t /= p;
                        e += 1;
                    }
                    let r = e % 3;
                    if r == 1 {
                        // need p^2
                        // NOTE: spec says overflow makes behavior undefined; we avoid panics but
                        // do not attempt to define overflow semantics beyond the spec.
                        comp = comp.saturating_mul(p.saturating_mul(p));
                    } else if r == 2 {
                        // need p^1
                        comp = comp.saturating_mul(p);
                    }
                    if comp > COMP_LIMIT {
                        break;
                    }
                }
            }

            if comp > COMP_LIMIT {
                None
            } else {
                // Handle remaining factor t similarly to residual rule used in reduction.
                if t > 1 {
                    if is_perfect_square_u64(t) {
                        let r = (t as f64).sqrt() as u64;
                        // t = r^2 => exponent 2, need exponent 1 => multiply by r
                        comp = comp.saturating_mul(r);
                    } else {
                        // exponent 1, need exponent 2 => multiply by t^2
                        comp = comp.saturating_mul(t.saturating_mul(t));
                    }
                }
                if comp > COMP_LIMIT {
                    None
                } else {
                    Some(comp)
                }
            }
        };

        let cnt_v = *counts.get(&v).unwrap_or(&0);
        if cnt_v == 0 {
            processed.insert(v);
            continue;
        }

        match cv_opt {
            None => {
                result += cnt_v;
                processed.insert(v);
            }
            Some(c) => {
                if let Some(&cnt_c) = counts.get(&c) {
                    if !processed.contains(&c) {
                        result += cnt_v.max(cnt_c);
                        processed.insert(v);
                        processed.insert(c);
                    } else {
                        // Complement already accounted for; v must not contribute again.
                        processed.insert(v);
                    }
                } else {
                    result += cnt_v;
                    processed.insert(v);
                }
            }
        }
    }

    print!("{result}");
}
