use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Info {
    no: i32,
    score: i32,
}

/// Parse next i32 from tokens vector at position `pos`.
/// Returns None if no more tokens are available.
fn next_i32(tokens: &[i32], pos: &mut usize) -> Option<i32> {
    if *pos >= tokens.len() {
        None
    } else {
        let v = tokens[*pos];
        *pos += 1;
        Some(v)
    }
}

/// Scoring logic translated from the C implementation.
/// Consumes integers via the `tokens` slice and advancing `pos`.
/// Returns None if input runs out (EOF in original program semantics).
///
/// The implementation intentionally mirrors the original control flow and
/// flag semantics (flgSpare, flgStrike, time, sum, frame) to preserve exact
/// scoring behavior including edge cases.
fn scoring(tokens: &[i32], pos: &mut usize) -> Option<i32> {
    let mut total: i32 = 0;
    let mut sum: i32 = 0;
    let mut frame: i32 = 1;
    let mut time: i32 = 1;
    let mut flg_spare: i32 = 0;
    let mut flg_strike: i32 = 0;

    while frame < 10 {
        let x = next_i32(tokens, pos)?;
        sum += x;

        if flg_spare != 0 {
            total += x;
            flg_spare = 0;
        } else if flg_strike != 0 {
            // Mirror original C logic:
            // if flgStrike == 3 then add x once and decrement,
            // then always add x and decrement again.
            if flg_strike == 3 {
                total += x;
                flg_strike -= 1;
            }
            total += x;
            flg_strike -= 1;
        }

        if time == 2 {
            total += sum;
            if sum == 10 {
                flg_spare = 1;
            }
            sum = 0;
            time = 1;
            frame += 1;
        } else if time == 1 {
            time += 1;
            if x == 10 {
                total += 10;
                flg_strike += 2;
                sum = 0;
                time = 1;
                frame += 1;
            }
        }
    }

    // Tenth frame and potential extra rolls (up to 3 reads).
    for i in 0..3 {
        if i == 2 && sum < 10 {
            break;
        }

        let x = next_i32(tokens, pos)?;
        total += x;
        sum += x;

        if flg_spare != 0 {
            total += x;
            flg_spare = 0;
        } else if flg_strike != 0 {
            if flg_strike == 3 {
                total += x;
                flg_strike -= 1;
            }
            total += x;
            flg_strike -= 1;
        }
    }

    Some(total)
}

fn main() {
    // Read entire stdin and parse all whitespace-separated integers into a Vec<i32>.
    // This matches the scanf-based input model: integers separated by whitespace.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, exit silently (mirrors termination on scanf failure).
        return;
    }

    // Parse tokens as i32. Any token that fails to parse will be ignored
    // by the filter_map; this approximates scanf behavior where non-integers
    // would cause scanf to fail and the program to stop reading further datasets.
    let tokens_vec: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut pos: usize = 0;

    // Outer loop: repeatedly read dataset size n; stop when n == 0 or no more input.
    loop {
        let n = match next_i32(&tokens_vec, &mut pos) {
            Some(v) => v,
            None => break, // EOF or no more ints: stop processing
        };
        if n == 0 {
            break;
        }

        // If n is unexpectedly large, we still proceed; spec expects n <= 40.
        let mut infos: Vec<Info> = Vec::with_capacity(n as usize);

        // Read n players: id and scoring rolls for each.
        let mut incomplete = false;
        for _ in 0..n {
            let id = match next_i32(&tokens_vec, &mut pos) {
                Some(v) => v,
                None => {
                    incomplete = true;
                    break;
                }
            };
            let score = match scoring(&tokens_vec, &mut pos) {
                Some(s) => s,
                None => {
                    incomplete = true;
                    break;
                }
            };
            infos.push(Info { no: id, score });
        }

        if incomplete {
            // Input ended prematurely. Mirror the original behavior by stopping further processing.
            break;
        }

        // Sort by score descending, then id ascending for ties.
        infos.sort_by(|a, b| {
            match b.score.cmp(&a.score) {
                std::cmp::Ordering::Equal => a.no.cmp(&b.no),
                other => other,
            }
        });

        // Print each player's id and score on its own line, matching "%d %d\n".
        for info in infos {
            println!("{} {}", info.no, info.score);
        }
    }
}
