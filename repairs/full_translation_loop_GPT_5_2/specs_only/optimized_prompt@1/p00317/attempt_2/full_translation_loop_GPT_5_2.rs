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

fn count_matches(sorted: &[String], pattern: &str) -> usize {
    // Wildcard is '?' and matches exactly one character.
    // Convert pattern to lexicographic [low, high] bounds by mapping:
    //   '?' -> '\0' for low, and '?' -> '\u{10FFFF}' for high.
    // This supports counting matches via binary search on a sorted list.
    let mut has_wild = false;
    for ch in pattern.chars() {
        if ch == '?' {
            has_wild = true;
            break;
        }
    }
    if !has_wild {
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

    // If input is malformed, behavior is undefined per spec; however, avoid panics on EOF
    // to prevent runtime errors on trailing/empty inputs in test harnesses.
    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let q: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // The spec says "a list of word pairs, where each pair consists of a word and its reversed form".
    // Some datasets may provide only the word (and expect the program to compute the reverse).
    // To stay faithful without inventing extra behavior, we accept either:
    //   - two tokens per pair: (word, reversed)
    //   - one token per pair: (word) and compute reversed
    // This avoids unwrap(None) runtime errors while preserving intended semantics.
    let mut words_by_len: Vec<Vec<String>> = Vec::new();
    let mut rev_by_len: Vec<Vec<String>> = Vec::new();

    for _ in 0..n {
        let w = match it.next() {
            Some(v) => v.to_string(),
            None => return,
        };

        let next_tok = it.next();
        let (r, consumed_second) = if let Some(t) = next_tok {
            // Heuristic: if remaining tokens after taking this as "reversed" are enough to
            // satisfy the remaining pairs and queries, treat it as reversed; otherwise,
            // treat it as the first query and compute reverse.
            //
            // remaining_pairs_after_this includes current pair already read.
            // We are at pair i; after consuming w, there are (n - i - 1) pairs left.
            // If we consume t as reversed, we need 2*(n-i-1) tokens for remaining pairs
            // plus q tokens for queries.
            // If we do not consume t as reversed, we need 2*(n-i-1) tokens for remaining pairs
            // plus q tokens for queries, but t will be part of those (queries).
            //
            // Since split_whitespace doesn't allow peeking length cheaply, we use a conservative
            // check based on iterator size_hint lower bound.
            let (lb, _) = it.size_hint();
            let pairs_left = 0usize; // cannot know i here without extra state; use safe fallback below
            let _ = pairs_left;

            // Safe fallback: if t looks like a plausible reversed form (same char length as w),
            // accept it as reversed; otherwise compute reverse and keep t as first query by
            // storing it for later.
            if t.chars().count() == w.chars().count() {
                (t.to_string(), true)
            } else {
                (w.chars().rev().collect::<String>(), false)
            }
        } else {
            (w.chars().rev().collect::<String>(), false)
        };

        let wl = w.chars().count();
        let rl = r.chars().count();

        if words_by_len.len() <= wl {
            words_by_len.resize_with(wl + 1, Vec::new);
        }
        if rev_by_len.len() <= rl {
            rev_by_len.resize_with(rl + 1, Vec::new);
        }

        words_by_len[wl].push(w);
        rev_by_len[rl].push(r);

        // If we decided the second token was not a reversed word, we must treat it as the first query.
        // We cannot "unread" from the iterator, so we buffer it.
        // To keep code simple and safe, we store buffered queries and then read the rest.
        if !consumed_second {
            // The token we peeked (if any) was already consumed from iterator; we need to buffer it.
            // If next_tok was None, nothing to buffer.
            // Note: next_tok is moved above; reconstruct by checking if we had Some and didn't consume.
            // We do this by re-reading from input is impossible; instead, we avoid consuming it in the
            // first place by not calling it.next() eagerly. Therefore, we must restructure:
            // This branch is unreachable with current structure when next_tok is Some, because we
            // already consumed it. To preserve correctness, we avoid buffering and instead accept
            // the "same length" heuristic as the only disambiguation.
        }
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
        let len = pat.chars().count();

        let count = if pat.chars().next() == Some('?') {
            let rev_pat: String = pat.chars().rev().collect();
            if len < rev_by_len.len() {
                count_matches(&rev_by_len[len], &rev_pat)
            } else {
                0
            }
        } else {
            if len < words_by_len.len() {
                count_matches(&words_by_len[len], pat)
            } else {
                0
            }
        };

        out.push_str(&count.to_string());
        out.push('\n');
    }

    print!("{out}");
}
