use std::io::{self, Read, Write};

/// Program translated from the given Specification.
///
/// Behavior highlights (kept close to the Specification):
/// - Read up to 100 pairs in the exact "%d,%d" style:
///     * Leading whitespace is allowed before the first %d (as scanf does).
///     * After the first integer, the very next byte MUST be ',' (no spaces allowed).
///     * After the comma, leading whitespace is allowed before the second %d.
/// - Stop reading pairs when (team + score) == 0 (sentinel) or 100 pairs read or a parse failure occurs.
/// - Sort records by score in nonincreasing order.
/// - Assign dense ranks: equal scores share the same rank; ranks increment by 1 upon strictly lower score.
/// - From the remaining input (after the initial phase), read whitespace-separated integers as queries until EOF.
///   For each query integer q, print one line per stored record with team == q; lines printed in the sorted record order.
///
/// Notes on implementation choices:
/// - We read the entire stdin into memory and parse bytes sequentially so we can faithfully split the input into
///   "initial pairs" and "queries" phases while mimicking scanf("%d,%d") semantics regarding the mandatory comma.
/// - Integer parsing uses i64 accumulation and then casts to i32 using `as i32` (wrap-around behavior similar to C
///   truncation is preserved if large values occur).
/// - If the initial-phase parsing fails before any records are read, this implementation exits without printing anything.
///   The Specification declares that case as "undefined behavior"; choosing to exit quietly is a narrow, safe interpretation.
/// - The program avoids unsafe Rust.
///
/// The program prints ranks (one integer per line) to stdout for each matching stored record per query.

#[derive(Clone, Copy)]
struct Record {
    team: i32,
    score: i32,
    rank: i32,
}

fn is_space(b: u8) -> bool {
    // ASCII whitespace as in C isspace for common whitespace chars:
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C | 0x0B)
}

/// Parse an integer at position `i` in `buf`.
/// If `skip_leading_ws` is true, skip ASCII whitespace before the integer (mimics %d).
/// Returns Some((value_as_i32, new_index)) on success, or None on failure (e.g. no digits, EOF).
///
/// The function accepts an optional '+' or '-' sign and requires at least one digit.
/// It does not skip trailing whitespace; caller inspects the byte after the integer as needed.
fn parse_int(buf: &[u8], mut i: usize, skip_leading_ws: bool) -> Option<(i32, usize)> {
    let n = buf.len();
    if skip_leading_ws {
        while i < n && is_space(buf[i]) {
            i += 1;
        }
    }
    if i >= n {
        return None;
    }
    // sign
    let mut neg = false;
    if buf[i] == b'+' {
        i += 1;
    } else if buf[i] == b'-' {
        neg = true;
        i += 1;
    }
    // need at least one digit
    let start_digits = i;
    let mut acc: i64 = 0;
    while i < n {
        let b = buf[i];
        if b'0' <= b && b <= b'9' {
            acc = acc * 10 + (b - b'0') as i64;
            i += 1;
        } else {
            break;
        }
    }
    if i == start_digits {
        // no digits found
        return None;
    }
    if neg {
        acc = -acc;
    }
    // cast to i32 (wrap/truncate like C would for out-of-range values)
    let val = acc as i32;
    Some((val, i))
}

/// Parse whitespace-separated integer token from `buf` starting at `i`.
/// This expects the token to be a contiguous non-whitespace run representing an integer,
/// i.e., optional sign and digits only. Returns Some((value, new_index_after_token)) or None on parse failure.
fn parse_int_token(buf: &[u8], mut i: usize) -> Option<(i32, usize)> {
    let n = buf.len();
    // skip leading whitespace first
    while i < n && is_space(buf[i]) {
        i += 1;
    }
    if i >= n {
        return None;
    }
    let start = i;
    // token extends until next whitespace
    while i < n && !is_space(buf[i]) {
        i += 1;
    }
    // parse token buf[start..i] as integer: must be optional sign + digits only
    let slice = &buf[start..i];
    if slice.is_empty() {
        return None;
    }
    let mut j = 0usize;
    if slice[j] == b'+' || slice[j] == b'-' {
        j += 1;
    }
    if j >= slice.len() {
        return None; // sign only, no digits
    }
    let mut acc: i64 = 0;
    while j < slice.len() {
        let b = slice[j];
        if b'0' <= b && b <= b'9' {
            acc = acc * 10 + (b - b'0') as i64;
            j += 1;
        } else {
            return None; // invalid char in token
        }
    }
    if slice[0] == b'-' {
        acc = -acc;
    }
    Some((acc as i32, i))
}

fn main() {
    // Read all stdin
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // On read error, exit (no output). This keeps behavior narrow.
        return;
    }

    let mut idx: usize = 0;
    let n = input.len();
    let mut records: Vec<Record> = Vec::with_capacity(100);

    // Initial records reading loop (up to 100)
    while records.len() < 100 {
        // parse first integer with leading whitespace skipped
        let first = parse_int(&input, idx, true);
        let (team, next_idx) = match first {
            Some(v) => v,
            None => break, // parse failure or EOF: stop reading initial pairs
        };
        idx = next_idx;

        // Next character must be ',' exactly (no whitespace allowed between integer and comma)
        if idx >= n || input[idx] != b',' {
            break; // format mismatch: stop reading initial pairs
        }
        idx += 1; // consume comma

        // parse second integer; %d after comma skips leading whitespace
        let second = parse_int(&input, idx, true);
        let (score, next_idx2) = match second {
            Some(v) => v,
            None => break, // parse failure: stop initial phase
        };
        idx = next_idx2;

        // sentinel check: stop when team + score == 0; sentinel not stored
        let sum = (team as i64) + (score as i64);
        if sum == 0 {
            break;
        }

        records.push(Record {
            team,
            score,
            rank: 0, // will fill later
        });
    }

    // If no records were stored, Specification deems behavior undefined.
    // Choosing a narrow, safe behavior: exit quietly without printing anything.
    if records.is_empty() {
        return;
    }

    // Sort by score nonincreasing (descending)
    records.sort_by(|a, b| b.score.cmp(&a.score));

    // Assign dense ranks
    let mut current_rank: i32 = 1;
    records[0].rank = current_rank;
    for i in 1..records.len() {
        if records[i].score == records[i - 1].score {
            records[i].rank = current_rank;
        } else {
            current_rank += 1;
            records[i].rank = current_rank;
        }
    }

    // Query phase: from current idx to EOF, parse whitespace-separated integers
    // For each query q, print rank lines for every stored record with team == q, in sorted order.
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut j = idx;
    loop {
        match parse_int_token(&input, j) {
            Some((q, new_j)) => {
                // For each stored record in order, print rank if team matches
                for rec in &records {
                    if rec.team == q {
                        // Print "<rank>\n"
                        // We ignore write errors and continue
                        let _ = writeln!(out, "{}", rec.rank);
                    }
                }
                j = new_j;
                if j >= n {
                    break;
                }
            }
            None => break, // no more tokens or parse failure => end of queries
        }
    }
}
