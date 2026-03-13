use std::io::{self, Read};

fn is_space(b: u8) -> bool {
    // ASCII whitespace similar to C's isspace for our purposes
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

/// Parse an integer from bytes starting at idx, skipping leading whitespace.
/// Returns Some((value_as_i32, new_index)) or None if EOF or no digits found.
/// Implements parsing of optional '+'/'-' and decimal digits. Values are parsed
/// into i64 then cast to i32 (wrapping) to emulate typical 32-bit C int behavior.
fn parse_int_skip_ws(buf: &[u8], mut idx: usize) -> Option<(i32, usize)> {
    let len = buf.len();
    // skip whitespace
    while idx < len && is_space(buf[idx]) {
        idx += 1;
    }
    if idx >= len {
        return None;
    }
    // optional sign
    let mut sign = 1i64;
    if buf[idx] == b'+' {
        idx += 1;
    } else if buf[idx] == b'-' {
        sign = -1;
        idx += 1;
    }
    // at least one digit required
    if idx >= len || !buf[idx].is_ascii_digit() {
        return None;
    }
    let mut val: i64 = 0;
    while idx < len && buf[idx].is_ascii_digit() {
        val = val * 10 + (buf[idx] - b'0') as i64;
        idx += 1;
    }
    val = val * sign;
    // cast to i32 with wrapping to approximate typical C 32-bit behavior
    let v32 = val as i32;
    Some((v32, idx))
}

fn main() {
    // Read entire stdin
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).expect("read stdin");

    // storage
    let mut team = [0i32; 100];
    let mut score = [0i32; 100];
    let mut rank = [0i32; 100];

    let mut idx: usize = 0;
    let mut count: usize = 0;

    // Read up to 100 pairs using scanf("%d,%d", &team[i], &score[i]);
    for i in 0..100 {
        // parse first %d (skips whitespace)
        let parsed_first = parse_int_skip_ws(&input, idx);
        if parsed_first.is_none() {
            break;
        }
        let (t, new_idx) = parsed_first.unwrap();
        idx = new_idx;

        // next character must be a literal ',' (no skipping)
        if idx >= input.len() || input[idx] != b',' {
            // scanf would fail; emulate stopping reading further pairs
            break;
        }
        idx += 1; // consume the comma

        // parse second %d (skips whitespace)
        let parsed_second = parse_int_skip_ws(&input, idx);
        if parsed_second.is_none() {
            break;
        }
        let (s, new_idx2) = parsed_second.unwrap();
        idx = new_idx2;

        team[i] = t;
        score[i] = s;

        if t.wrapping_add(s) == 0 {
            // stop reading pairs when team+score == 0
            // This matches the C code's check, using wrapping_add to emulate possible wrap behavior.
            // Note: In typical inputs this is just t==0 && s==0.
            break;
        }
        count += 1;
    }

    // Sorting: replicate the exact nested loops and condition used in C:
    // for(i=0;i<count;i++) { for(j=count-1;j>i;j--) { if(score[i]<=score[j]) swap } }
    for i in 0..count {
        for j in ((i + 1)..count).rev() {
            if score[i] <= score[j] {
                score.swap(i, j);
                team.swap(i, j);
            }
        }
    }

    // Ranking
    if count > 0 {
        let mut number: i32 = 1;
        rank[0] = number;
        let mut tmp2 = score[0];
        for i in 1..count {
            if score[i] != tmp2 {
                number = number.wrapping_add(1); // mimic potential wrap, though unlikely needed
                tmp2 = score[i];
            }
            rank[i] = number;
        }
    }

    // Queries: while(scanf("%d",&info)!=EOF) { ... }
    // Continue parsing integers from current idx until EOF
    while let Some((info, new_idx)) = parse_int_skip_ws(&input, idx) {
        idx = new_idx;
        for i in 0..count {
            if info == team[i] {
                // Print rank with newline, matching printf("%d\n", rank[i]);
                println!("{}", rank[i]);
            }
        }
    }
}
