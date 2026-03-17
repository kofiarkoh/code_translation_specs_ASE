use std::io::{self, BufRead};
use std::cmp::Ordering;

const MAX_WORDS: usize = 50000;
const MAX_LENGTH: usize = 204;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut words: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        let word = lines.next().unwrap().unwrap();
        words.push(word);
    }

    // Sort the words for binary search
    words.sort();

    for _ in 0..m {
        let pattern = lines.next().unwrap().unwrap();
        let count = count_matching_words(&words, &pattern);
        println!("{}", count);
    }
}

fn count_matching_words(words: &[String], pattern: &str) -> usize {
    let n = pattern.len();
    let mut wildcard_pos = None;

    // Find the position of the first wildcard
    for (i, c) in pattern.chars().enumerate() {
        if c == '?' {
            wildcard_pos = Some(i);
            break;
        }
    }

    let is_prefix = pattern.starts_with('*');
    let is_suffix = pattern.ends_with('*');

    let mut count = 0;

    if is_prefix {
        let trimmed_pattern = &pattern[1..];
        if let Some(pos) = wildcard_pos {
            if pos >= n - 1 {
                return words.len(); // All words match if wildcard is at the end
            }
            for c in 'a'..='z' {
                let mut modified_pattern = trimmed_pattern.to_string();
                modified_pattern.replace_range(pos..pos + 1, &c.to_string());
                count += binary_search(&words, &modified_pattern, is_suffix) as usize;
            }
        } else {
            count = binary_search(&words, trimmed_pattern, is_suffix) as usize;
        }
    } else if is_suffix {
        let trimmed_pattern = &pattern[..n - 1];
        if let Some(pos) = wildcard_pos {
            if pos == 0 {
                return words.len(); // All words match if wildcard is at the start
            }
            for c in 'a'..='z' {
                let mut modified_pattern = trimmed_pattern.to_string();
                modified_pattern.replace_range(pos..pos + 1, &c.to_string());
                count += binary_search(&words, &modified_pattern, false) as usize;
            }
        } else {
            count = binary_search(&words, trimmed_pattern, false) as usize;
        }
    } else {
        if let Some(pos) = wildcard_pos {
            for c in 'a'..='z' {
                let mut modified_pattern = pattern.to_string();
                modified_pattern.replace_range(pos..pos + 1, &c.to_string());
                count += binary_search(&words, &modified_pattern, false) as usize;
            }
        } else {
            count = binary_search(&words, pattern, false) as usize;
        }
    }

    count
}

fn binary_search(words: &[String], pattern: &str, is_suffix: bool) -> usize {
    let n = words.len();
    let mut low = 0;
    let mut high = n;

    if is_suffix {
        while low < high {
            let mid = (low + high) / 2;
            if words[mid] < pattern {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let upper_bound = low;

        low = 0;
        high = n;
        while low < high {
            let mid = (low + high) / 2;
            if words[mid] <= pattern {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        return upper_bound - low;
    } else {
        while low < high {
            let mid = (low + high) / 2;
            match words[mid].cmp(pattern) {
                Ordering::Equal => return 1,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
    }
    0
}
