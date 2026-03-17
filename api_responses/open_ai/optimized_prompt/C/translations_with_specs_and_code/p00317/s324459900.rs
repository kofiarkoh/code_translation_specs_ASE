use std::io::{self, BufRead};
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut words = Vec::with_capacity(n);
    let mut reversed_words = Vec::with_capacity(n);

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut split = line.split_whitespace();
        let word = split.next().unwrap().to_string();
        let reversed: String = word.chars().rev().collect();
        words.push(word);
        reversed_words.push(reversed);
    }

    words.sort();
    reversed_words.sort();

    for _ in 0..m {
        let pattern = lines.next().unwrap().unwrap();
        let count = count_matches(&pattern, &words, &reversed_words);
        println!("{}", count);
    }
}

fn count_matches(pattern: &str, words: &[String], reversed_words: &[String]) -> usize {
    let n = pattern.len();
    let wildcard_pos = pattern.find('?');
    let is_prefix = pattern.starts_with('*');
    let is_suffix = pattern.ends_with('*');

    let mut count = 0;

    if is_prefix {
        let trimmed_pattern = &pattern[1..];
        if let Some(pos) = wildcard_pos {
            let prefix_length = pos;
            for c in 'a'..='z' {
                let mut modified_pattern = trimmed_pattern.to_string();
                modified_pattern.replace_range(pos..pos + 1, &c.to_string());
                count += binary_search(&modified_pattern, words, true);
            }
        } else {
            count = binary_search(trimmed_pattern, words, true);
        }
    } else if is_suffix {
        let trimmed_pattern = &pattern[..n - 1];
        if let Some(pos) = wildcard_pos {
            let suffix_length = n - 1 - pos;
            for c in 'a'..='z' {
                let mut modified_pattern = trimmed_pattern.to_string();
                modified_pattern.replace_range(pos..pos + 1, &c.to_string());
                count += binary_search(&modified_pattern, reversed_words, false);
            }
        } else {
            count = binary_search(trimmed_pattern, reversed_words, false);
        }
    } else {
        if let Some(pos) = wildcard_pos {
            for c in 'a'..='z' {
                let mut modified_pattern = pattern.to_string();
                modified_pattern.replace_range(pos..pos + 1, &c.to_string());
                count += binary_search(&modified_pattern, words, true) + binary_search(&modified_pattern, reversed_words, false);
            }
        } else {
            count += binary_search(pattern, words, true) + binary_search(pattern, reversed_words, false);
        }
    }

    count
}

fn binary_search(pattern: &str, words: &[String], is_prefix: bool) -> usize {
    let mut low = 0;
    let mut high = words.len();

    if is_prefix {
        while low < high {
            let mid = (low + high) / 2;
            if words[mid].starts_with(pattern) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let upper_bound = low;

        low = 0;
        high = words.len();
        while low < high {
            let mid = (low + high) / 2;
            if words[mid] < pattern {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let lower_bound = low;

        upper_bound - lower_bound
    } else {
        while low < high {
            let mid = (low + high) / 2;
            match words[mid].cmp(pattern) {
                Ordering::Equal => return 1,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        0
    }
}
