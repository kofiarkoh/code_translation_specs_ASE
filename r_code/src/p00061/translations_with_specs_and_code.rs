use std::io::{self, Read};

fn main() {
    // Read entire stdin to a buffer and parse sequentially.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let bytes = input.as_bytes();
    let mut pos: usize = 0;
    let len = bytes.len();

    // Helper: skip ASCII whitespace
    let mut skip_whitespace = |pos: &mut usize| {
        while *pos < len && bytes[*pos].is_ascii_whitespace() {
            *pos += 1;
        }
    };

    // Parse an integer with behavior similar to scanf("%d"):
    // - skips leading whitespace
    // - accepts optional '+' or '-' sign
    // - requires at least one digit
    // On failure returns None and does not advance pos beyond the point of failure except for consumed characters.
    let mut parse_int = |pos: &mut usize| -> Option<i32> {
        // skip leading whitespace
        skip_whitespace(pos);
        if *pos >= len {
            return None;
        }
        // optional sign
        let mut sign: i64 = 1;
        if bytes[*pos] == b'+' {
            *pos += 1;
        } else if bytes[*pos] == b'-' {
            sign = -1;
            *pos += 1;
        }
        if *pos >= len {
            return None;
        }
        // at least one digit required
        if !bytes[*pos].is_ascii_digit() {
            return None;
        }
        let mut val: i64 = 0;
        while *pos < len && bytes[*pos].is_ascii_digit() {
            val = val * 10 + (bytes[*pos] - b'0') as i64;
            *pos += 1;
        }
        // Cast to i32 as program expects 'int'
        Some((val * sign) as i32)
    };

    // Parse a pair matching the scanf pattern "%d,%d":
    // - skips leading whitespace before first %d
    // - after first integer, expects comma immediately (no intervening whitespace)
    // - after comma, the second %d skips leading whitespace
    let mut parse_pair = |pos: &mut usize| -> Option<(i32, i32)> {
        // parse first integer (skipping leading whitespace)
        let team = {
            // Use a temporary local pos to detect failure without partial consumption beyond what parse_int does.
            let res = parse_int(pos)?;
            res
        };
        // After parse_int, the position is at the first non-digit character. For scanf("%d,%d"),
        // the next character must be a literal comma with no whitespace between number and comma.
        if *pos >= len || bytes[*pos] != b',' {
            return None;
        }
        // consume comma
        *pos += 1;
        // parse second integer (skipping whitespace as scanf would for %d after a literal)
        let score = parse_int(pos)?;
        Some((team, score))
    };

    // Read up to 100 pairs, stop on sentinel team+score == 0 or when parsing fails / EOF reached.
    let mut records: Vec<(i32, i32)> = Vec::new(); // (team, score)
    for _ in 0..100 {
        match parse_pair(&mut pos) {
            Some((team, score)) => {
                if team + score == 0 {
                    break; // sentinel detected: do not store sentinel pair
                }
                records.push((team, score));
            }
            None => break, // parsing failed or EOF: stop reading initial records
        }
    }

    // If no records were read, behavior in the original C program is undefined.
    // Here we handle this gracefully by skipping ranking and queries (no output).
    if records.is_empty() {
        return;
    }

    // Sort by score in nonincreasing (descending) order.
    records.sort_by(|a, b| b.1.cmp(&a.1));

    // Assign dense ranks: equal scores share the same rank; start at 1 for highest score.
    let mut ranks: Vec<i32> = Vec::with_capacity(records.len());
    let mut current_rank: i32 = 1;
    ranks.push(current_rank);
    let mut prev_score = records[0].1;
    for i in 1..records.len() {
        let score = records[i].1;
        if score != prev_score {
            current_rank += 1;
            prev_score = score;
        }
        ranks.push(current_rank);
    }

    // Query phase: parse remaining whitespace-separated integers until EOF.
    // For each query integer, print the rank for every stored record whose team id equals the queried id.
    loop {
        // parse_int skips leading whitespace; if none left or next token not an int, stop.
        match parse_int(&mut pos) {
            Some(query) => {
                for (idx, (team, _score)) in records.iter().enumerate() {
                    if *team == query {
                        println!("{}", ranks[idx]);
                    }
                }
            }
            None => break,
        }
    }
}
