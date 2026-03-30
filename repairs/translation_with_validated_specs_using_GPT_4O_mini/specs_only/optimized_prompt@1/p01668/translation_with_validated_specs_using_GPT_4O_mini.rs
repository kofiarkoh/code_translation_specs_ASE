use std::collections::HashMap;

const MODULO: i64 = 1_000_000_007;

fn count_valid_sequences(a: &str, b: &str, c: &str) -> i64 {
    let a = pad_with_zeros(a, b.len());
    let b = b.to_string();
    let c = c.to_string();
    
    let mut memo = HashMap::new();
    count_sequences_helper(&a, &b, &c, 0, 1, 1, &mut memo)
}

fn count_sequences_helper(
    a: &str,
    b: &str,
    c: &str,
    pos: usize,
    tight_a: usize,
    tight_b: usize,
    memo: &mut HashMap<(usize, usize, usize), i64>,
) -> i64 {
    if pos == c.len() {
        return 1; // Found a valid sequence
    }
    
    if let Some(&result) = memo.get(&(pos, tight_a, tight_b)) {
        return result; // Return cached result
    }

    let lower_bound = if tight_a == 1 { a.chars().nth(pos).unwrap() } else { '0' };
    let upper_bound = if tight_b == 1 { b.chars().nth(pos).unwrap() } else { '9' };

    let mut count = 0;

    for digit in lower_bound..=upper_bound {
        if digit == c.chars().nth(pos).unwrap() {
            count += count_sequences_helper(a, b, c, pos + 1, tight_a, tight_b, memo);
        } else if digit < c.chars().nth(pos).unwrap() {
            count += count_sequences_helper(a, b, c, pos + 1, 0, tight_b, memo);
        } else {
            count += count_sequences_helper(a, b, c, pos + 1, tight_a, 0, memo);
        }
        count %= MODULO; // Ensure we stay within bounds
    }

    memo.insert((pos, tight_a, tight_b), count);
    count
}

fn pad_with_zeros(s: &str, target_len: usize) -> String {
    let mut padded = String::new();
    let padding_needed = target_len.saturating_sub(s.len());
    padded.push_str(&"0".repeat(padding_needed));
    padded.push_str(s);
    padded
}

fn main() {
    let a = "123";
    let b = "456";
    let c = "12";
    
    let result = count_valid_sequences(a, b, c);
    println!("{}", result); // Output the count of valid sequences
}
