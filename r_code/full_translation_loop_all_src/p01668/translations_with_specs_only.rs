use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn mod_add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

fn mod_sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

fn build_kmp_pi(pat: &[u8]) -> Vec<usize> {
    let m = pat.len();
    let mut pi = vec![0usize; m];
    let mut j = 0usize;
    for i in 1..m {
        while j > 0 && pat[i] != pat[j] {
            j = pi[j - 1];
        }
        if pat[i] == pat[j] {
            j += 1;
            pi[i] = j;
        }
    }
    pi
}

fn build_kmp_next(pat: &[u8], pi: &[usize]) -> Vec<[usize; 10]> {
    let m = pat.len();
    let mut next = vec![[0usize; 10]; m + 1];
    for state in 0..=m {
        for d in 0..10usize {
            if state == m {
                // Once matched, we keep it matched (absorbing).
                next[state][d] = m;
                continue;
            }
            let ch = b'0' + d as u8;
            let mut j = state;
            while j > 0 && pat[j] != ch {
                j = pi[j - 1];
            }
            if pat[j] == ch {
                j += 1;
            }
            next[state][d] = j;
        }
    }
    next
}

// Counts N-digit sequences X with 0 <= X <= bound (bound is N digits),
// such that pattern occurs as a substring starting at some s >= start_min.
// All arithmetic modulo MOD.
fn count_up_to(bound: &[u8], start_min: usize, next: &[[usize; 10]], m: usize) -> u64 {
    let n = bound.len();
    // dp_tight[state][started] and dp_loose[state][started]
    // started: whether we've already started a match at some s >= start_min.
    let mut tight = vec![[0u64; 2]; m + 1];
    let mut loose = vec![[0u64; 2]; m + 1];
    tight[0][0] = 1;

    for i in 0..n {
        let limit = (bound[i] - b'0') as usize;
        let mut ntight = vec![[0u64; 2]; m + 1];
        let mut nloose = vec![[0u64; 2]; m + 1];

        // Transition from tight
        for state in 0..=m {
            for started in 0..=1usize {
                let ways = tight[state][started];
                if ways == 0 {
                    continue;
                }
                for d in 0..=limit {
                    let ns = next[state][d];
                    let mut nstarted = started;
                    // A match "starts" at position i - m + 1 when we first reach ns == m.
                    // We only allow counting it if that start index >= start_min.
                    if started == 0 && ns == m {
                        let start_idx = i + 1 - m;
                        if start_idx >= start_min {
                            nstarted = 1;
                        }
                    }
                    if d == limit {
                        ntight[ns][nstarted] = mod_add(ntight[ns][nstarted], ways);
                    } else {
                        nloose[ns][nstarted] = mod_add(nloose[ns][nstarted], ways);
                    }
                }
            }
        }

        // Transition from loose
        for state in 0..=m {
            for started in 0..=1usize {
                let ways = loose[state][started];
                if ways == 0 {
                    continue;
                }
                for d in 0..10usize {
                    let ns = next[state][d];
                    let mut nstarted = started;
                    if started == 0 && ns == m {
                        let start_idx = i + 1 - m;
                        if start_idx >= start_min {
                            nstarted = 1;
                        }
                    }
                    nloose[ns][nstarted] = mod_add(nloose[ns][nstarted], ways);
                }
            }
        }

        tight = ntight;
        loose = nloose;
    }

    let mut ans = 0u64;
    for state in 0..=m {
        ans = mod_add(ans, tight[state][1]);
        ans = mod_add(ans, loose[state][1]);
    }
    ans
}

// Subtracts 1 from a non-empty decimal digit string (ASCII bytes), in-place.
// Returns false if underflow would occur (i.e., input is all zeros), true otherwise.
fn dec_sub_one(s: &mut [u8]) -> bool {
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s[i] > b'0' {
            s[i] -= 1;
            return true;
        } else {
            s[i] = b'9';
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();
    let low = match it.next() {
        Some(s) => s.as_bytes().to_vec(),
        None => std::process::exit(1),
    };
    let up = match it.next() {
        Some(s) => s.as_bytes().to_vec(),
        None => std::process::exit(1),
    };
    let pat = match it.next() {
        Some(s) => s.as_bytes().to_vec(),
        None => std::process::exit(1),
    };

    let n = up.len();
    let low_len = low.len();
    let start_min = n - low_len;

    // Pad lower bound to length N with leading zeros.
    let mut low_pad = vec![b'0'; n];
    low_pad[n - low_len..].copy_from_slice(&low);

    let m = pat.len();
    let pi = build_kmp_pi(&pat);
    let next = build_kmp_next(&pat, &pi);

    // Count in [0, up] minus count in [0, low_pad - 1].
    let up_count = count_up_to(&up, start_min, &next, m);

    let mut low_minus_one = low_pad.clone();
    let has_prev = dec_sub_one(&mut low_minus_one);
    let low_count = if has_prev {
        count_up_to(&low_minus_one, start_min, &next, m)
    } else {
        0u64
    };

    let ans = mod_sub(up_count, low_count);
    println!("{}", ans);
}
