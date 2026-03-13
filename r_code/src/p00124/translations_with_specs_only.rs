use std::io::{self, Read};

const MAX_N: usize = 10;
const MAX_NAME_LEN: usize = 10;

/// A record holding the (possibly truncated) name, the computed score (wrapping as i32),
/// and the original input index within the dataset to preserve stability for equal scores.
#[derive(Debug)]
struct Record {
    name: String,
    score: i32,
    index: usize,
}

fn truncate_name(name: &str) -> String {
    // Truncate by characters to avoid cutting UTF-8 codepoints in half.
    // The specification expects names up to 10 characters (C bytes),
    // but truncating by chars is a conservative, safe choice.
    name.chars().take(MAX_NAME_LEN).collect()
}

fn main() {
    // Read entire stdin and tokenize by whitespace.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit silently (spec does not prescribe error handling).
        return;
    }
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut pos: usize = 0;
    let mut dataset_count: usize = 0;

    // Helper to parse next token as i32. Returns None if token not available or parse fails.
    let mut parse_next_i32 = |pos: &mut usize| -> Option<i32> {
        if *pos >= tokens.len() {
            return None;
        }
        let t = tokens[*pos];
        *pos += 1;
        match t.parse::<i32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    };

    while pos < tokens.len() {
        // Read n
        let n = match parse_next_i32(&mut pos) {
            Some(v) => v,
            None => break, // malformed or EOF: stop processing (spec: undefined, but we stop)
        };

        // Termination condition
        if n == 0 {
            break;
        }

        let mut records: Vec<Record> = Vec::new();
        let mut input_index: usize = 0;

        if n > 0 {
            // Read n records; even if n > MAX_N we will consume all tokens but store at most MAX_N records.
            let n_usize = n as usize;
            for _ in 0..n_usize {
                // Each record expects: name a b c
                if pos >= tokens.len() {
                    // insufficient tokens: stop processing entirely
                    // (spec: undefined; we choose to stop)
                    pos = tokens.len();
                    break;
                }
                let name_tok = tokens[pos];
                pos += 1;

                // parse a, b, c as i32
                let a = match parse_next_i32(&mut pos) {
                    Some(v) => v,
                    None => {
                        pos = tokens.len();
                        break;
                    }
                };
                let _b = match parse_next_i32(&mut pos) {
                    Some(v) => v,
                    None => {
                        pos = tokens.len();
                        break;
                    }
                };
                let c = match parse_next_i32(&mut pos) {
                    Some(v) => v,
                    None => {
                        pos = tokens.len();
                        break;
                    }
                };

                // Compute score = 3*a + c using wrapping arithmetic to emulate C int wraparound behavior.
                let score = a.wrapping_mul(3).wrapping_add(c);

                // Store only up to MAX_N records to reflect the specification's supported capacity.
                if records.len() < MAX_N {
                    records.push(Record {
                        name: truncate_name(name_tok),
                        score,
                        index: input_index,
                    });
                }
                input_index = input_index.wrapping_add(1);
            }
        } else {
            // n < 0: per specification, loops that iterate up to n will not execute,
            // so no records are read/consumed for this dataset; the dataset effectively produces no output,
            // but it is still considered a processed dataset (affects blank-line separation).
            // Therefore, do nothing here.
        }

        // Sort records by descending score, preserving original order for equal scores by comparing index.
        records.sort_by(|a, b| {
            // First compare score descending
            match b.score.cmp(&a.score) {
                std::cmp::Ordering::Equal => a.index.cmp(&b.index),
                ord => ord,
            }
        });

        // Print a separating blank line between consecutive dataset outputs (no leading blank line).
        if dataset_count > 0 {
            println!();
        }

        // Print each record as "name,score"
        for rec in records.iter() {
            println!("{},{}", rec.name, rec.score);
        }

        dataset_count = dataset_count.wrapping_add(1);
    }

    // Normal termination -> implicit exit with code 0
}
