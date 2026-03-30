use std::io::{self, BufRead};
use std::cmp::Ordering;

const MAX_WORDS: usize = 50000;
const MAX_LENGTH: usize = 204;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let n: usize = lines.next().unwrap().parse().expect("Failed to parse N");
    let m: usize = lines.next().unwrap().parse().expect("Failed to parse M");
    
    let mut words = Vec::with_capacity(n);
    let mut reversed_words = Vec::with_capacity(n);
    
    for _ in 0..n {
        let word = lines.next().unwrap();
        let reversed: String = word.chars().rev().collect();
        words.push(word);
        reversed_words.push(reversed);
    }
    
    words.sort();
    reversed_words.sort();
    
    for _ in 0..m {
        let query = lines.next().unwrap();
        let query_length = query.len();
        
        let (mut part, mut f_res) = (0, 0);
        let mut wildcard_pos = query.find('?');
        
        let mut s = query.as_str();
        let mut n = query_length;

        if query.starts_with('*') {
            part = 1;
            n -= 1;
            s = &query[1..];
            if wildcard_pos.is_some() {
                wildcard_pos = Some(wildcard_pos.unwrap() - 1);
            }
        } else if query.ends_with('*') {
            part = 1;
            n -= 1;
            s = &query[..query_length - 1];
            f_res = 1;
            if wildcard_pos.is_some() {
                wildcard_pos = Some(wildcard_pos.unwrap() + 1);
            }
        }

        let k = wildcard_pos.map_or(-1, |pos| pos as isize);
        let ans = if k >= 0 {
            if n == 1 {
                if part == 1 { n } else { words.len() }
            } else {
                let mut count = 0;
                for c in 'a'..='z' {
                    let mut temp_query = s.to_string();
                    temp_query.replace_range(k as usize..=k as usize, &c.to_string());
                    count += search(&temp_query, n, f_res, part, &words, &reversed_words);
                }
                count
            }
        } else {
            search(s, n, f_res, part, &words, &reversed_words)
        };
        
        println!("{}", ans);
    }
}

fn search(query: &str, n: usize, f_res: usize, part: usize, words: &[String], reversed_words: &[String]) -> usize {
    let mut low = 0;
    let mut high = if part == 1 { words.len() } else { words.len() };

    if part == 1 {
        while low < high {
            let mid = (low + high) / 2;
            let comparison = if f_res == 1 {
                reversed_words[mid].as_str().cmp(&query[..n])
            } else {
                words[mid].as_str().cmp(&query[..n])
            };
            match comparison {
                Ordering::Less | Ordering::Equal => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        let k = low as isize - 1;
        low = 0;
        high = if part == 1 { words.len() } else { words.len() };
        while low < high {
            let mid = (low + high) / 2;
            let comparison = if f_res == 1 {
                reversed_words[mid].as_str().cmp(&query[..n])
            } else {
                words[mid].as_str().cmp(&query[..n])
            };
            match comparison {
                Ordering::Less => low = mid + 1,
                Ordering::Greater | Ordering::Equal => high = mid,
            }
        }
        (k - low as isize + 1) as usize
    } else {
        while low < high {
            let mid = (low + high) / 2;
            let comparison = if f_res == 1 {
                reversed_words[mid].as_str().cmp(query)
            } else {
                words[mid].as_str().cmp(query)
            };
            match comparison {
                Ordering::Equal => return 1,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        0
    }
}
