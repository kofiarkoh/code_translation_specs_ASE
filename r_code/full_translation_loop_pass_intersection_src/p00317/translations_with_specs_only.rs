use std::cmp::Ordering;
use std::io::{self, Read};

fn lower_bound(arr: &[Vec<u8>], key: &[u8]) -> usize {
    let mut l = 0usize;
    let mut r = arr.len();
    while l < r {
        let m = (l + r) / 2;
        match arr[m].as_slice().cmp(key) {
            Ordering::Less => l = m + 1,
            _ => r = m,
        }
    }
    l
}

fn upper_bound(arr: &[Vec<u8>], key: &[u8]) -> usize {
    let mut l = 0usize;
    let mut r = arr.len();
    while l < r {
        let m = (l + r) / 2;
        match arr[m].as_slice().cmp(key) {
            Ordering::Greater => r = m,
            _ => l = m + 1,
        }
    }
    l
}

fn count_prefix(sorted: &[Vec<u8>], prefix: &[u8]) -> usize {
    // Count words w where w starts with prefix, using lexicographic range:
    // [prefix, prefix + 0xFF] in bytewise ordering.
    let mut hi = Vec::with_capacity(prefix.len() + 1);
    hi.extend_from_slice(prefix);
    hi.push(0xFF);
    let lo_i = lower_bound(sorted, prefix);
    let hi_i = upper_bound(sorted, &hi);
    hi_i.saturating_sub(lo_i)
}

fn count_exact(sorted: &[Vec<u8>], key: &[u8]) -> usize {
    let lo = lower_bound(sorted, key);
    let hi = upper_bound(sorted, key);
    hi.saturating_sub(lo)
}

fn reverse_bytes(s: &[u8]) -> Vec<u8> {
    let mut v = s.to_vec();
    v.reverse();
    v
}

fn first_qmark_pos(pat: &[u8]) -> Option<usize> {
    pat.iter().position(|&c| c == b'?')
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut dict: Vec<Vec<u8>> = Vec::with_capacity(n);
    let mut rdict: Vec<Vec<u8>> = Vec::with_capacity(n);

    for _ in 0..n {
        let w = it.next().unwrap_or("").as_bytes().to_vec();
        rdict.push(reverse_bytes(&w));
        dict.push(w);
    }

    dict.sort();
    rdict.sort();

    let mut out = String::new();

    for _ in 0..m {
        let pat_str = it.next().unwrap_or("");
        let pat = pat_str.as_bytes();

        // Determine star form. Specification says only one '*' at one end is supported.
        let (use_reversed, is_prefix_query, core): (bool, bool, &[u8]) = if pat.first() == Some(&b'*') {
            // Leading '*': suffix match => reverse and do prefix match on reversed.
            (true, true, &pat[1..])
        } else if pat.last() == Some(&b'*') {
            // Trailing '*': prefix match.
            (false, true, &pat[..pat.len().saturating_sub(1)])
        } else {
            // No '*': exact match.
            (false, false, pat)
        };

        let (sorted, core_bytes): (&[Vec<u8>], Vec<u8>) = if use_reversed {
            (rdict.as_slice(), reverse_bytes(core))
        } else {
            (dict.as_slice(), core.to_vec())
        };

        // Only the first '?' is expanded; remaining '?' are treated literally.
        let qpos = first_qmark_pos(&core_bytes);

        let mut total: usize = 0;

        if let Some(pos) = qpos {
            // Expand '?' into 26 lowercase letters.
            for c in b'a'..=b'z' {
                let mut candidate = core_bytes.clone();
                candidate[pos] = c;

                let cnt = if is_prefix_query {
                    count_prefix(sorted, &candidate)
                } else {
                    count_exact(sorted, &candidate)
                };
                total = total.wrapping_add(cnt);
            }
        } else {
            total = if is_prefix_query {
                count_prefix(sorted, &core_bytes)
            } else {
                count_exact(sorted, &core_bytes)
            };
        }

        out.push_str(&format!("{}\n", total));
    }

    print!("{}", out);
}
