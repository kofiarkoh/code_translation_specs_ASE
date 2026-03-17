use std::collections::HashSet;
use std::io::{self, BufRead};

fn reverse(s: &mut [char], n: usize) {
    let mid = n / 2;
    for i in 0..mid {
        s.swap(i, n - 1 - i);
    }
}

fn swap(s1: &mut [char], s2: &mut [char]) {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut t = Vec::with_capacity(len1 + len2);
    t.extend_from_slice(s2);
    t.extend_from_slice(s1);
    s1.copy_from_slice(&t[..len1]);
}

fn contain(variations: &HashSet<String>, s: &str) -> bool {
    variations.contains(s)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let max_variations = 1024;

    for _ in 0..n {
        let s: String = lines.next().unwrap().unwrap();
        let len = s.len();
        let mut variations = HashSet::new();
        let mut t: Vec<char> = s.chars().collect();

        for i in 1..len {
            if !contain(&variations, &s) {
                variations.insert(s.clone());
            }

            reverse(&mut t, i);
            if !contain(&variations, &t.iter().collect::<String>()) {
                variations.insert(t.iter().collect());
            }

            reverse(&mut t[i..], len - i);
            if !contain(&variations, &t.iter().collect::<String>()) {
                variations.insert(t.iter().collect());
            }

            reverse(&mut t, i);
            if !contain(&variations, &t.iter().collect::<String>()) {
                variations.insert(t.iter().collect());
            }

            t = s.chars().collect();
            let mut s1: Vec<char> = t.clone();
            let mut s2: Vec<char> = t[i..].to_vec();
            swap(&mut s1, &mut s2);

            if !contain(&variations, &s1.iter().collect::<String>()) {
                variations.insert(s1.iter().collect());
            }

            reverse(&mut s1[len - i..], i);
            if !contain(&variations, &s1.iter().collect::<String>()) {
                variations.insert(s1.iter().collect());
            }

            reverse(&mut s1[len - i..], i);
            if !contain(&variations, &s1.iter().collect::<String>()) {
                variations.insert(s1.iter().collect());
            }

            reverse(&mut s1, len - i);
            if !contain(&variations, &s1.iter().collect::<String>()) {
                variations.insert(s1.iter().collect());
            }

            t = s.chars().collect();
        }

        println!("{}", variations.len());
    }
}
