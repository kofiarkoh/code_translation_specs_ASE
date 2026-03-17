use std::io::{self, Write};
use std::str;

const MAX_STRINGS: usize = 1024;
const MAX_LENGTH: usize = 80;

struct StringContainer {
    strings: [[u8; MAX_LENGTH]; MAX_STRINGS],
    count: usize,
}

impl StringContainer {
    fn new() -> Self {
        StringContainer {
            strings: [[0; MAX_LENGTH]; MAX_STRINGS],
            count: 0,
        }
    }

    fn contains(&self, s: &[u8]) -> bool {
        for i in 0..self.count {
            if self.strings[i] == s {
                return true;
            }
        }
        false
    }

    fn add(&mut self, s: &[u8]) {
        if self.count < MAX_STRINGS {
            self.strings[self.count].copy_from_slice(s);
            self.count += 1;
        }
    }
}

fn reverse(s: &mut [u8], n: usize) {
    for i in 0..n / 2 {
        s.swap(i, n - 1 - i);
    }
}

fn swap(s1: &mut [u8], s2: &mut [u8]) {
    let mut t = Vec::new();
    t.extend_from_slice(s2);
    t.extend_from_slice(s1);
    t.push(0); // Null-terminate

    let len = t.len();
    s1.copy_from_slice(&t[..len]);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut container = StringContainer::new();
        let mut s = [0; MAX_LENGTH];
        let mut t = [0; MAX_LENGTH];

        io::stdin().read_line(&mut input).unwrap();
        let len = input.trim().as_bytes().len();
        s[..len].copy_from_slice(input.trim().as_bytes());
        t.copy_from_slice(&s);

        for i in 1..len {
            if !container.contains(&s[..len]) {
                container.add(&s[..len]);
            }
            reverse(&mut s[..i], i);
            if !container.contains(&s[..len]) {
                container.add(&s[..len]);
            }
            reverse(&mut s[i..len], len - i);
            if !container.contains(&s[..len]) {
                container.add(&s[..len]);
            }
            reverse(&mut s[..i], i);
            s.copy_from_slice(&t);

            swap(&mut s[..], &mut s[i..]);
            if !container.contains(&s[..len]) {
                container.add(&s[..len]);
            }
            reverse(&mut s[len - i..len], i);
            if !container.contains(&s[..len]) {
                container.add(&s[..len]);
            }
            reverse(&mut s[..len - i], len - i);
            if !container.contains(&s[..len]) {
                container.add(&s[..len]);
            }
            s.copy_from_slice(&t);
        }
        println!("{}", container.count);
    }
}
