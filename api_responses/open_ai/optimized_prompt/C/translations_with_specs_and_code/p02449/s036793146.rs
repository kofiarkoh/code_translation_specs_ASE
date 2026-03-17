use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be an integer");

    // Preconditions check
    if n < 1 || n > 9 {
        return; // Behavior is not defined for n outside [1, 9]
    }

    let mut sequence = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.trim().split_whitespace() {
        let value: i32 = num.parse().expect("Input must be an integer");
        sequence.push(value);
    }

    // Previous permutation
    let mut prev_permutation = sequence.clone();
    if n == 1 {
        println!("1");
    } else {
        if let Some(prev) = previous_permutation(&mut prev_permutation) {
            println!("{}", prev.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
        }
    }

    // Print the original sequence
    println!("{}", sequence.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));

    // Next permutation
    let mut next_permutation = sequence.clone();
    if let Some(next) = next_permutation(&mut next_permutation) {
        println!("{}", next.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}

fn previous_permutation(seq: &mut Vec<i32>) -> Option<Vec<i32>> {
    let n = seq.len();
    let mut i = n - 2;

    while i as isize >= 0 && seq[i] <= seq[i + 1] {
        i -= 1;
    }

    if i == isize::from(-1) {
        return None; // No previous permutation
    }

    let mut j = n - 1;
    while seq[j] >= seq[i] {
        j -= 1;
    }

    seq.swap(i, j);
    seq[i + 1..].reverse();

    Some(seq.clone())
}

fn next_permutation(seq: &mut Vec<i32>) -> Option<Vec<i32>> {
    let n = seq.len();
    let mut i = n - 2;

    while i as isize >= 0 && seq[i] >= seq[i + 1] {
        i -= 1;
    }

    if i == isize::from(-1) {
        return None; // No next permutation
    }

    let mut j = n - 1;
    while seq[j] <= seq[i] {
        j -= 1;
    }

    seq.swap(i, j);
    seq[i + 1..].reverse();

    Some(seq.clone())
}
