use std::cmp::Ordering;
use std::io::{self, Read};

fn lower_bound(arr: &[String], key: &str) -> usize {
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].as_str().cmp(key) {
            Ordering::Less => lo = mid + 1,
            _ => hi = mid,
        }
    }
    lo
}

fn upper_bound(arr: &[String], key: &str) -> usize {
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].as_str().cmp(key) {
            Ordering::Greater => hi = mid,
            _ => lo = mid + 1,
        }
    }
    lo
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn count_matches_question(sorted: &[String], pattern: &str) -> usize {
    // '?' matches exactly one character.
    // Count matches by converting to lexicographic [low, high] bounds.
    let mut has_q = false;
    for ch in pattern.chars() {
        if ch == '?' {
            has_q = true;
            break;
        }
    }
    if !has_q {
        let l = lower_bound(sorted, pattern);
        let r = upper_bound(sorted, pattern);
        return r - l;
    }

    let mut low = String::with_capacity(pattern.len());
    let mut high = String::with_capacity(pattern.len());
    for ch in pattern.chars() {
        if ch == '?' {
            low.push('\0');
            high.push('\u{10FFFF}');
        } else {
            low.push(ch);
            high.push(ch);
        }
    }

    let l = lower_bound(sorted, &low);
    let r = upper_bound(sorted, &high);
    r - l
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let q: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // The observed dataset provides only N words (not explicit word+reversed pairs).
    // We therefore read exactly N words and compute their reversed forms.
    let mut words_by_len: Vec<Vec<String>> = Vec::new();
    let mut rev_by_len: Vec<Vec<String>> = Vec::new();

    for _ in 0..n {
        let w = match it.next() {
            Some(v) => v.to_string(),
            None => return,
        };
        let r = reverse_string(&w);

        let wl = w.len(); // treat as bytes; typical problems use lowercase ASCII
        if words_by_len.len() <= wl {
            words_by_len.resize_with(wl + 1, Vec::new);
            rev_by_len.resize_with(wl + 1, Vec::new);
        }
        words_by_len[wl].push(w);
        rev_by_len[wl].push(r);
    }

    for v in &mut words_by_len {
        v.sort();
    }
    for v in &mut rev_by_len {
        v.sort();
    }

    let mut out = String::new();
    for _ in 0..q {
        let pat = match it.next() {
            Some(v) => v,
            None => break,
        };

        // Support two wildcard forms:
        //  - '?' matches exactly one character (fixed-length patterns)
        //  - '*' matches any sequence (including empty), but in this problem it is used as a
        //    contiguous block at the start or end (prefix/suffix queries).
        //
        // Narrow interpretation to match typical intended semantics:
        //  - If pattern contains '*', it is treated as a single contiguous block and the rest
        //    is a fixed prefix or suffix.
        //  - Otherwise, use '?' logic.
        if pat.contains('*') {
            // Determine if '*' is leading or trailing (or all '*').
            let bytes = pat.as_bytes();
            let mut first_non_star = 0usize;
            while first_non_star < bytes.len() && bytes[first_non_star] == b'*' {
                first_non_star += 1;
            }
            let mut last_non_star = bytes.len();
            while last_non_star > 0 && bytes[last_non_star - 1] == b'*' {
                last_non_star -= 1;
            }

            if first_non_star >= last_non_star {
                // Pattern is all '*' => matches all words.
                let mut total = 0usize;
                for v in &words_by_len {
                    total += v.len();
                }
                out.push_str(&total.to_string());
                out.push('\n');
                continue;
            }

            let core = &pat[first_non_star..last_non_star];
            if first_non_star == 0 && last_non_star < bytes.len() {
                // Trailing '*' => prefix match on normal words.
                let core_len = core.len();
                if core_len >= words_by_len.len() {
                    out.push_str("0\n");
                    continue;
                }
                let vec = &words_by_len[core_len];
                // For prefix match with variable tail, count all words of length >= core_len
                // that start with core. Since we bucket by exact length, we must sum over all
                // lengths >= core_len.
                let mut total = 0usize;
                for len in core_len..words_by_len.len() {
                    let v = &words_by_len[len];
                    if v.is_empty() {
                        continue;
                    }
                    // Range: [core, core_with_last_char_max...]
                    let mut low = String::with_capacity(len);
                    low.push_str(core);
                    // For upper bound, append max char to core and then allow any suffix by
                    // using '\u{10FFFF}' padding; but since we compare full strings, we can
                    // use prefix range by constructing:
                    //   low = core
                    //   high = core + '\u{10FFFF}' repeated
                    // This works because all strings in v have same length.
                    let mut high = String::with_capacity(len);
                    high.push_str(core);
                    for _ in core_len..len {
                        high.push('\u{10FFFF}');
                    }
                    // For low, pad with '\0' to ensure it is the smallest string with that prefix.
                    for _ in core_len..len {
                        low.push('\0');
                    }
                    let l = lower_bound(v, &low);
                    let r = upper_bound(v, &high);
                    total += r - l;
                }
                out.push_str(&total.to_string());
                out.push('\n');
            } else if first_non_star > 0 && last_non_star == bytes.len() {
                // Leading '*' => suffix match; use reversed words and reversed core as prefix.
                let core_rev = reverse_string(core);
                let core_len = core_rev.len();
                if core_len >= rev_by_len.len() {
                    out.push_str("0\n");
                    continue;
                }
                let mut total = 0usize;
                for len in core_len..rev_by_len.len() {
                    let v = &rev_by_len[len];
                    if v.is_empty() {
                        continue;
                    }
                    let mut low = String::with_capacity(len);
                    low.push_str(&core_rev);
                    let mut high = String::with_capacity(len);
                    high.push_str(&core_rev);
                    for _ in core_len..len {
                        low.push('\0');
                        high.push('\u{10FFFF}');
                    }
                    let l = lower_bound(v, &low);
                    let r = upper_bound(v, &high);
                    total += r - l;
                }
                out.push_str(&total.to_string());
                out.push('\n');
            } else {
                // '*' in the middle (not just leading/trailing) is not specified; choose the
                // narrowest behavior: do a direct scan over all words (still correct).
                let mut total = 0usize;
                for v in &words_by_len {
                    for w in v {
                        if wildcard_match_star(pat, w) {
                            total += 1;
                        }
                    }
                }
                out.push_str(&total.to_string());
                out.push('\n');
            }
        } else {
            // '?' fixed-length matching; choose direction based on leading '?' (suffix query).
            let len = pat.len();
            let count = if pat.as_bytes().first() == Some(&b'?') {
                let rev_pat = reverse_string(pat);
                if len < rev_by_len.len() {
                    count_matches_question(&rev_by_len[len], &rev_pat)
                } else {
                    0
                }
            } else {
                if len < words_by_len.len() {
                    count_matches_question(&words_by_len[len], pat)
                } else {
                    0
                }
            };
            out.push_str(&count.to_string());
            out.push('\n');
        }
    }

    print!("{out}");
}

fn wildcard_match_star(pat: &str, text: &str) -> bool {
    // Minimal '*' glob matcher (supports '?' too) for fallback when '*' appears in the middle.
    // This is a standard linear-time greedy algorithm.
    let p = pat.as_bytes();
    let t = text.as_bytes();
    let (mut i, mut j) = (0usize, 0usize);
    let mut star: Option<usize> = None;
    let mut match_i = 0usize;

    while j < t.len() {
        if i < p.len() && (p[i] == b'?' || p[i] == t[j]) {
            i += 1;
            j += 1;
        } else if i < p.len() && p[i] == b'*' {
            star = Some(i);
            i += 1;
            match_i = j;
        } else if let Some(si) = star {
            i = si + 1;
            match_i += 1;
            j = match_i;
        } else {
            return false;
        }
    }
    while i < p.len() && p[i] == b'*' {
        i += 1;
    }
    i == p.len()
}
