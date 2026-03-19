use std::io::{self, Read};

const M: i64 = 998_244_353;
const MAX_SUM: usize = 90310;

fn main() {
    // Read entire stdin, parse as integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // SPEC: if initial integer cannot be read, behavior is not inferable.
    // We mimic C's "unspecified" by just returning without output.
    let first = match it.next() {
        Some(x) => x.parse::<usize>().unwrap(),
        None => return,
    };

    // d[k] = D(k), e[k] = E(k) as in the spec, modulo M.
    // Fixed-size arrays as in C (bounded by MAX_SUM).
    let mut d = vec![0i64; MAX_SUM];
    let mut e = vec![0i64; MAX_SUM];

    // C initializes *d and *e to scanf("%d") return value (1 on success).
    // Since we successfully read the first integer, set d[0]=e[0]=1.
    d[0] = 1;
    e[0] = 1;

    // Running total sum and 3^n factor.
    let mut s: usize = 0;
    let mut r: i64 = 1;

    // Process the first integer and all subsequent integers uniformly.
    // In C, the loop reads subsequent ints; the first int is only used to seed d/e.
    // But the spec defines the sequence includes the initial integer, so we must
    // incorporate it as an element too. The C code effectively does incorporate
    // every read integer *after* seeding; the initial integer value itself is not
    // used in updates. This is a mismatch: spec says x0 is part of the multiset.
    // However, the C code's behavior is the only implementable reference here.
    // To preserve C semantics, we treat the initial integer as "present only to
    // indicate successful read" and do NOT add it as an item.
    //
    // NOTE: This explicitly follows C over spec due to direct conflict.
    // If you want spec behavior, the first value should be processed like others.

    // Read remaining integers until EOF.
    while let Some(tok) = it.next() {
        let a: usize = tok.parse().unwrap();

        // Update subset-sum aggregates for new value a.
        // for (t = s; t >= 0; --t) { d[t+a] += d[t]*2; e[t+a] += e[t]; }
        // All modulo M.
        for t in (0..=s).rev() {
            let idx = t + a;
            // Preconditions guarantee idx < MAX_SUM.
            d[idx] = (d[idx] + (d[t] * 2) % M) % M;
            e[idx] = (e[idx] + e[t]) % M;
        }

        s += a;
        r = (r * 3) % M;
    }

    // Finalization:
    // r = 3^n - 3 * sum_{k=0..floor(s/2)} d[k]  (mod M)
    for k in 0..=s / 2 {
        r = (r - (d[k] * 3) % M) % M;
    }

    // Then adjust by - (~s%2) * e[s/2] * 3, where (~s%2) is -1 if s even else 0.
    // So this adds +3*e[s/2] when s is even, else adds 0.
    if s % 2 == 0 {
        r = (r + (e[s / 2] * 3) % M) % M;
    }

    // Canonicalize to [0, M-1] and print as integer with no extra whitespace.
    r %= M;
    if r < 0 {
        r += M;
    }
    print!("{}", r);
}
