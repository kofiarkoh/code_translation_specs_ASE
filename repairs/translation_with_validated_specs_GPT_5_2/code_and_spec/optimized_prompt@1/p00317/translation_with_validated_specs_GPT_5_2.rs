use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    // Read entire stdin (whitespace-delimited tokens, like the C code).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    // Dictionary as multiset: keep all words (including duplicates).
    // Also keep reversed words for suffix queries (leading '*').
    let mut w: Vec<String> = Vec::with_capacity(n);
    let mut r: Vec<String> = Vec::with_capacity(n);
    let mut one = 0usize;

    for _ in 0..n {
        let word = it.next().unwrap().to_string();
        if word.len() == 1 {
            one += 1;
        }
        let rev: String = word.chars().rev().collect();
        w.push(word);
        r.push(rev);
    }

    w.sort_unstable();
    r.sort_unstable();

    // Binary search helper:
    // - part == false: exact match existence (returns 1 or 0)
    // - part == true: count of strings having prefix `s[..len]` (returns multiplicity)
    fn search(sorted: &[String], s: &str, len: usize, part: bool) -> i32 {
        if !part {
            return if sorted.binary_search_by(|x| x.as_str().cmp(s)).is_ok() {
                1
            } else {
                0
            };
        }

        // Count strings where x[..] starts with s[0..len] (byte-wise).
        // This mirrors the C code's use of strncmp with length `len`.
        let prefix = &s.as_bytes()[..len];

        // upper_bound: first index where x_prefix > prefix
        let mut low = 0usize;
        let mut high = sorted.len();
        while low < high {
            let mid = (low + high) >> 1;
            let xb = sorted[mid].as_bytes();
            let cmp = cmp_prefix(xb, prefix);
            if cmp <= Ordering::Equal {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let upper = low; // exclusive

        // lower_bound: first index where x_prefix >= prefix
        low = 0;
        high = sorted.len();
        while low < high {
            let mid = (low + high) >> 1;
            let xb = sorted[mid].as_bytes();
            let cmp = cmp_prefix(xb, prefix);
            if cmp == Ordering::Less {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let lower = low;

        (upper as i32) - (lower as i32)
    }

    // Compare xb[0..len] with prefix (like strncmp(x, prefix, len)).
    // If xb is shorter than len, treat missing bytes as '\0' (C strncmp behavior).
    fn cmp_prefix(xb: &[u8], prefix: &[u8]) -> Ordering {
        let len = prefix.len();
        for i in 0..len {
            let a = if i < xb.len() { xb[i] } else { 0 };
            let b = prefix[i];
            if a < b {
                return Ordering::Less;
            } else if a > b {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }

    let mut out = String::new();

    for _ in 0..m {
        let pat = it.next().unwrap().to_string();
        let bytes = pat.as_bytes();

        // Find first '?' only; remaining '?' are treated literally (per spec/C behavior).
        let mut qpos: isize = -1;
        for (i, &ch) in bytes.iter().enumerate() {
            if ch == b'?' {
                qpos = i as isize;
                break;
            }
        }

        // Determine whether this is a prefix/suffix query via '*' at one end.
        // C code uses the reversed copy to detect trailing '*' (in original) and leading '*'.
        let mut part = false;
        let mut use_reversed = false;

        // s_bytes is the pattern string we will search against (possibly reversed).
        // nlen is the effective length excluding '*' if present at an end.
        let mut s_bytes: Vec<u8>;
        let mut nlen: usize;

        // Reverse of pattern (like C's `res`).
        let mut rev_pat: Vec<u8> = bytes.iter().copied().rev().collect();

        if !rev_pat.is_empty() && rev_pat[0] == b'*' {
            // Original pattern ends with '*': prefix match on original words.
            part = true;
            nlen = bytes.len() - 1;
            s_bytes = bytes[..nlen].to_vec();
            if qpos as usize >= nlen {
                qpos = -1;
            }
        } else if !bytes.is_empty() && bytes[0] == b'*' {
            // Original pattern begins with '*': suffix match.
            // Use reversed dictionary and reversed suffix (excluding '*').
            part = true;
            use_reversed = true;
            nlen = bytes.len() - 1;
            // rev_pat currently is reverse(original). Removing trailing '*' in original
            // corresponds to removing last byte in rev_pat (which is '*').
            // But C does: res[--n]=0 where res is reversed, with n being original length-1.
            // Net effect: use rev_pat[0..nlen] (drop the final '*' from rev_pat).
            rev_pat.truncate(nlen);
            s_bytes = rev_pat;

            // Adjust qpos to index in reversed string, matching C's `k = n - k`.
            // C's `n` here is (original_len - 1) after removing '*'.
            // If '?' was at position k in original (0-based), in reversed (length nlen),
            // it becomes at index (nlen - 1 - k). C computes `n - k` with n = nlen,
            // which equals (nlen - k). That is one off from the usual formula, but
            // C also has different boundary checks; we preserve C's behavior.
            if qpos >= 0 {
                let k = qpos as usize;
                if k >= nlen + 1 {
                    qpos = -1;
                } else {
                    qpos = (nlen as isize) - (k as isize);
                }
            }
        } else {
            // No '*': exact match.
            part = false;
            nlen = bytes.len();
            s_bytes = bytes.to_vec();
            if qpos as usize >= nlen {
                qpos = -1;
            }
        }

        let dict = if use_reversed { &r } else { &w };

        let ans: i32;
        if qpos >= 0 {
            // If effective length is 1, C special-cases:
            // - part ? N : one
            // This matches spec: '?' expands to any letter; for exact match length 1,
            // count words of length 1; for prefix/suffix with length 1 (just '?'),
            // every word matches.
            if nlen == 1 {
                ans = if part { n as i32 } else { one as i32 };
            } else {
                let k = qpos as usize;
                let mut total = 0i32;
                for ch in b'a'..=b'z' {
                    s_bytes[k] = ch;
                    // Interpret s_bytes as UTF-8; inputs are assumed lowercase/ASCII per spec.
                    let s = std::str::from_utf8(&s_bytes).unwrap();
                    total += search(dict, s, nlen, part);
                }
                ans = total;
            }
        } else {
            let s = std::str::from_utf8(&s_bytes).unwrap();
            ans = search(dict, s, nlen, part);
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
