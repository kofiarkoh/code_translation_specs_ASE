use std::collections::HashSet;
use std::io::{self, Read};

fn take_prefix_token(line: &str) -> &str {
    // Only the contiguous prefix up to (but not including) the first whitespace is considered.
    // If the line is empty or starts with whitespace, this yields an empty string.
    line.split_whitespace().next().unwrap_or("")
}

fn signature_for_window(bytes: &[u8], start: usize, k: usize) -> [u16; 26] {
    // Preconditions guarantee lowercase a-z in the considered prefix.
    // Use u16 counts; supported maximum line length is within fixed buffers per spec.
    let mut sig = [0u16; 26];
    for &b in &bytes[start..start + k] {
        let idx = (b - b'a') as usize;
        sig[idx] = sig[idx].wrapping_add(1);
    }
    sig
}

fn has_anagrammatic_common_substring(a: &[u8], b: &[u8], k: usize) -> bool {
    if k == 0 {
        return true;
    }
    if a.len() < k || b.len() < k {
        return false;
    }

    // Membership set of distinct signatures from A's length-k substrings.
    // Spec mentions fixed-capacity membership structure; Rust's HashSet is dynamic,
    // but behaviorally it matches the required membership semantics.
    let mut set: HashSet<[u16; 26]> = HashSet::new();

    for i in 0..=a.len() - k {
        set.insert(signature_for_window(a, i, k));
    }

    for j in 0..=b.len() - k {
        let sig = signature_for_window(b, j, k);
        if set.contains(&sig) {
            return true;
        }
    }

    false
}

fn main() {
    // Read entire stdin; behavior is unspecified if inputs are missing/invalid.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let line1 = lines.next().unwrap_or("");
    let line2 = lines.next().unwrap_or("");

    let a_str = take_prefix_token(line1);
    let b_str = take_prefix_token(line2);

    let a = a_str.as_bytes();
    let b = b_str.as_bytes();

    let max_k = std::cmp::min(a.len(), b.len());
    let mut answer = 0usize;

    // Evaluate k from max down to 1; terminate early on first match.
    for k in (1..=max_k).rev() {
        if has_anagrammatic_common_substring(a, b, k) {
            answer = k;
            break;
        }
    }

    println!("{}", answer);
}
