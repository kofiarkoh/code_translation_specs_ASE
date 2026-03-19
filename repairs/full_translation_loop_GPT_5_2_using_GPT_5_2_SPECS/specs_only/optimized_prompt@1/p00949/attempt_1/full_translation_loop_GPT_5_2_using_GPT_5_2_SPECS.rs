use std::collections::HashSet;
use std::io::{self, Read};

const MAX_N: usize = 4003;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Sig {
    // 26 letter counts; u16 is sufficient for lengths up to 4003.
    c: [u16; 26],
}

fn parse_prefix_token(line: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for b in line.bytes() {
        if b.is_ascii_whitespace() {
            break;
        }
        // Preconditions say only 'a'..'z' appear; no validation per spec.
        out.push(b);
        if out.len() >= MAX_N {
            // Preconditions say at most 4003; if violated, behavior is not inferable.
            // We cap to fixed maximum to keep bounded storage consistent with spec intent.
            break;
        }
    }
    out
}

fn build_sigs(s: &[u8], len: usize) -> HashSet<Sig> {
    let n = s.len();
    let mut set = HashSet::new();
    if len == 0 {
        set.insert(Sig { c: [0u16; 26] });
        return set;
    }
    if len > n {
        return set;
    }

    let mut counts = [0u16; 26];
    for &b in &s[..len] {
        let idx = (b - b'a') as usize;
        counts[idx] = counts[idx].wrapping_add(1);
    }
    set.insert(Sig { c: counts });

    for i in len..n {
        let out_b = s[i - len];
        let in_b = s[i];
        let out_idx = (out_b - b'a') as usize;
        let in_idx = (in_b - b'a') as usize;

        counts[out_idx] = counts[out_idx].wrapping_sub(1);
        counts[in_idx] = counts[in_idx].wrapping_add(1);

        set.insert(Sig { c: counts });
    }

    set
}

fn has_common_anagram_substring(a: &[u8], b: &[u8], len: usize) -> bool {
    if len == 0 {
        return true;
    }
    if len > a.len() || len > b.len() {
        return false;
    }

    // Build signatures for the smaller side first to reduce memory.
    if a.len() <= b.len() {
        let set = build_sigs(a, len);
        if set.is_empty() {
            return false;
        }
        // Scan b and check membership without storing all b signatures.
        let n = b.len();
        let mut counts = [0u16; 26];
        for &bb in &b[..len] {
            let idx = (bb - b'a') as usize;
            counts[idx] = counts[idx].wrapping_add(1);
        }
        if set.contains(&Sig { c: counts }) {
            return true;
        }
        for i in len..n {
            let out_b = b[i - len];
            let in_b = b[i];
            let out_idx = (out_b - b'a') as usize;
            let in_idx = (in_b - b'a') as usize;

            counts[out_idx] = counts[out_idx].wrapping_sub(1);
            counts[in_idx] = counts[in_idx].wrapping_add(1);

            if set.contains(&Sig { c: counts }) {
                return true;
            }
        }
        false
    } else {
        has_common_anagram_substring(b, a, len)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let line1 = lines.next().unwrap_or("");
    let line2 = lines.next().unwrap_or("");

    let a = parse_prefix_token(line1);
    let b = parse_prefix_token(line2);

    let mut lo = 0usize;
    let mut hi = a.len().min(b.len());
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if has_common_anagram_substring(&a, &b, mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }

    println!("{}", lo);
}
