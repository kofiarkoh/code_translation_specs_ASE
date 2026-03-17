use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut sequence = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    sequence.extend(input.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()));

    if n == 1 {
        println!("1");
    } else {
        let mut prev_permutation = sequence.clone();
        let mut next_permutation = sequence.clone();

        // Compute previous permutation
        if let Some(prev) = previous_permutation(&mut prev_permutation) {
            println!("{}", prev.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
        }

        // Print the original sequence
        println!("{}", sequence.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));

        // Compute next permutation
        if let Some(next) = next_permutation(&mut next_permutation) {
            println!("{}", next.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
        }
    }
}

fn previous_permutation(seq: &mut Vec<i32>) -> Option<Vec<i32>> {
    let n = seq.len();
    let mut i = n as isize - 2;

    while i >= 0 && seq[i as usize] <= seq[(i + 1) as usize] {
        i -= 1;
    }

    if i == -1 {
        return None; // No previous permutation
    }

    let mut j = n - 1;
    while seq[j] >= seq[i as usize] {
        j -= 1;
    }

    seq.swap(i as usize, j);
    seq[(i + 1) as usize..].reverse();

    Some(seq.clone())
}

fn next_permutation(seq: &mut Vec<i32>) -> Option<Vec<i32>> {
    let n = seq.len();
    let mut i = n as isize - 2;

    while i >= 0 && seq[i as usize] >= seq[(i + 1) as usize] {
        i -= 1;
    }

    if i == -1 {
        return None; // No next permutation
    }

    let mut j = n - 1;
    while seq[j] <= seq[i as usize] {
        j -= 1;
    }

    seq.swap(i as usize, j);
    seq[(i + 1) as usize..].reverse();

    Some(seq.clone())
}
