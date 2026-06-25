use std::io::{self, Read};

fn main() {
    // Read entire stdin; specification assumes two non-empty whitespace-delimited tokens exist.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let s1 = it.next().unwrap();
    let s2 = it.next().unwrap();

    // Fixed-size internal storage per specification (bounded, constant memory).
    // Preconditions guarantee 2*|s1| <= 299, so this buffer is sufficient.
    let mut buf = [0u8; 300];

    let b1 = s1.as_bytes();
    let n1 = b1.len();
    for i in 0..n1 {
        buf[i] = b1[i];
        buf[i + n1] = b1[i];
    }
    let doubled = &buf[..(2 * n1)];

    let b2 = s2.as_bytes();
    let found = contains_subslice(doubled, b2);

    // Specification only requires an affirmative/negative response; choose minimal tokens.
    if found {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    // Preconditions state non-empty inputs, but handle empty needle narrowly and safely.
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    for i in 0..=haystack.len() - needle.len() {
        if &haystack[i..i + needle.len()] == needle {
            return true;
        }
    }
    false
}
