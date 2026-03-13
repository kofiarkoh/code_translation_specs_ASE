// drop_ink.rs
//
// Translated from the provided C program.
// Preserves semantics for typical well-formed input of comma-separated triples.
// Notes about scanning:
// - The original used scanf("%d,%d,%d", ... ) in a loop until EOF.
// - This translation implements a manual scanner that closely follows the
//   behavior for well-formed inputs and also handles a final partial triple
//   at EOF (e.g. "x,y" with no third value) by processing whatever conversions
//   succeeded, as the C scanf would do.
// - For malformed inputs (non-digit where a number is expected and not EOF)
//   the original program's scanf would return 0 and could loop; to avoid
//   pathological infinite loops here we will stop consuming input in that
//   case and terminate scanning. This matches expected use (well-formed data).
//
// Overflow semantics:
// - The C code used signed int increments. Signed overflow in C is undefined.
//   To avoid panics in Rust debug mode and to reflect typical two's-complement
//   machine behavior, increments use wrapping arithmetic (wrapping_add(1)).

use std::io::{self, Read};

fn main() {
    // map and in arrays as in C
    let mut map: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut input: [i32; 3] = [0; 3];

    // Read whole stdin
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read stdin");

    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut pos: usize = 0;

    // Helper closures for parsing
    // Skip whitespace like scanf's %d (which skips leading whitespace)
    let is_space = |b: u8| match b {
        b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c => true,
        _ => false,
    };

    // Parse an int at bytes[pos], advancing pos on success.
    // Returns Some(i32) if digits present (with optional leading sign),
    // or None if there were no digits at the current position (after skipping leading whitespace).
    // Parsing uses wrapping arithmetic to emulate typical C int behavior.
    let mut parse_int = |pos_ref: &mut usize| -> Option<i32> {
        // skip leading whitespace
        while *pos_ref < len && is_space(bytes[*pos_ref]) {
            *pos_ref += 1;
        }
        if *pos_ref >= len {
            return None;
        }
        // sign
        let mut neg = false;
        if bytes[*pos_ref] == b'+' {
            *pos_ref += 1;
        } else if bytes[*pos_ref] == b'-' {
            neg = true;
            *pos_ref += 1;
        }
        // require at least one digit
        if *pos_ref >= len || !(bytes[*pos_ref] as char).is_ascii_digit() {
            return None;
        }
        let mut val: i32 = 0;
        while *pos_ref < len {
            let c = bytes[*pos_ref];
            if !(c as char).is_ascii_digit() {
                break;
            }
            let digit = (c - b'0') as i32;
            // wrapping arithmetic to avoid panic on overflow, emulating typical machine wrap
            val = val.wrapping_mul(10).wrapping_add(digit);
            *pos_ref += 1;
        }
        if neg {
            val = (-val).wrapping_add(0); // wrap negation (preserve two's complement style)
        }
        Some(val)
    };

    // Main scanning loop attempting to emulate scanf("%d,%d,%d")
    loop {
        // skip leading whitespace (scanf's %d consumes leading whitespace)
        while pos < len && is_space(bytes[pos]) {
            pos += 1;
        }
        if pos >= len {
            // EOF before any conversion -> scanf would return EOF; stop loop.
            break;
        }

        // Attempt first %d
        let saved_pos = pos;
        match parse_int(&mut pos) {
            Some(v1) => {
                input[0] = v1;
            }
            None => {
                // No digits for first conversion: this would make scanf return 0 (not EOF).
                // The original program would then call drop_ink (because scanf != EOF),
                // but it would also likely get stuck repeatedly on the same bad input.
                // To avoid an infinite loop here we stop scanning entirely.
                // (This is a pragmatic divergence for malformed input.)
                break;
            }
        }

        // Next in format is a literal ',' which must match exactly (no skipping of whitespace).
        if pos >= len {
            // EOF after first conversion: scanf would return 1 (not EOF). Call drop_ink once and break.
            drop_ink(&mut map, &input);
            break;
        }
        if bytes[pos] != b',' {
            // mismatch: scanf would stop here and return number of conversions so far (1).
            // We'll emulate by calling drop_ink once and then advance by one to avoid stuck parsing.
            drop_ink(&mut map, &input);
            pos += 1;
            continue;
        }
        // consume comma
        pos += 1;

        // parse second %d (this %d will skip leading whitespace)
        match parse_int(&mut pos) {
            Some(v2) => {
                input[1] = v2;
            }
            None => {
                // If EOF reached while trying to parse second number, scanf returns 1 (we handled EOF above).
                // If a mismatch (non-digit) occurs, emulate calling drop_ink and advance to avoid infinite loop.
                // Check EOF:
                if pos >= len {
                    drop_ink(&mut map, &input);
                    break;
                } else {
                    drop_ink(&mut map, &input);
                    pos += 1;
                    continue;
                }
            }
        }

        // expect second comma literal
        if pos >= len {
            // EOF after two conversions: call drop_ink and break.
            drop_ink(&mut map, &input);
            break;
        }
        if bytes[pos] != b',' {
            // mismatch: call drop_ink and advance pos to avoid stuck parsing.
            drop_ink(&mut map, &input);
            pos += 1;
            continue;
        }
        pos += 1; // consume comma

        // parse third %d
        match parse_int(&mut pos) {
            Some(v3) => {
                input[2] = v3;
                // full triple parsed; call drop_ink and continue
                drop_ink(&mut map, &input);
                continue;
            }
            None => {
                // If EOF meanwhile, scanf would return 2 (not EOF) and call drop_ink once.
                if pos >= len {
                    drop_ink(&mut map, &input);
                    break;
                } else {
                    // mismatch: call drop_ink and advance to avoid stuck parsing.
                    drop_ink(&mut map, &input);
                    pos += 1;
                    continue;
                }
            }
        }
    }

    // After all input processed, print untaint() and mosttaint() results exactly as in C:
    // printf("%d\n%d\n", untaint(), mosttaint());
    let u = untaint(&map);
    let m = mosttaint(&map);
    // Use print! so we match exact formatting (two numbers each followed by newline).
    print!("{}\n{}\n", u, m);
}

// The translation of drop_ink and taint_* functions.
// Emulate the C switch with fall-through behavior.

fn drop_ink(map: &mut [[i32; 10]; 10], input: &[i32; 3]) {
    match input[2] {
        3 => {
            taint_3(map, input);
            // fallthrough
            taint_2(map, input);
            taint_1(map, input);
        }
        2 => {
            taint_2(map, input);
            taint_1(map, input);
        }
        1 => {
            taint_1(map, input);
        }
        _ => {
            // do nothing
        }
    }
}

fn taint_1(map: &mut [[i32; 10]; 10], input: &[i32; 3]) {
    let x = input[0];
    let y = input[1];
    // if (in[0] - 1 >= 0) map[in[0] - 1][in[1]]++;
    if x - 1 >= 0 {
        let xi = (x - 1) as usize;
        let yi = y as usize;
        if xi < 10 && yi < 10 {
            map[xi][yi] = map[xi][yi].wrapping_add(1);
        }
    }
    // if (in[0] + 1 < 10) map[in[0] + 1][in[1]]++;
    if x + 1 < 10 {
        let xi = (x + 1) as usize;
        let yi = y as usize;
        if xi < 10 && yi < 10 {
            map[xi][yi] = map[xi][yi].wrapping_add(1);
        }
    }
    // if (in[1] - 1 >= 0) map[in[0]][in[1] - 1]++;
    if y - 1 >= 0 {
        let xi = x as usize;
        let yi = (y - 1) as usize;
        if xi < 10 && yi < 10 {
            map[xi][yi] = map[xi][yi].wrapping_add(1);
        }
    }
    // if (in[1] + 1 < 10) map[in[0]][in[1] + 1]++;
    if y + 1 < 10 {
        let xi = x as usize;
        let yi = (y + 1) as usize;
        if xi < 10 && yi < 10 {
            map[xi][yi] = map[xi][yi].wrapping_add(1);
        }
    }
}

fn taint_2(map: &mut [[i32; 10]; 10], input: &[i32; 3]) {
    let x = input[0];
    let y = input[1];
    if x - 1 >= 0 {
        let xi = (x - 1) as usize;
        if y - 1 >= 0 {
            let yi = (y - 1) as usize;
            if xi < 10 && yi < 10 {
                map[xi][yi] = map[xi][yi].wrapping_add(1);
            }
        }
        if y + 1 < 10 {
            let yi = (y + 1) as usize;
            if xi < 10 && yi < 10 {
                map[xi][yi] = map[xi][yi].wrapping_add(1);
            }
        }
    }
    if x + 1 < 10 {
        let xi = (x + 1) as usize;
        if y - 1 >= 0 {
            let yi = (y - 1) as usize;
            if xi < 10 && yi < 10 {
                map[xi][yi] = map[xi][yi].wrapping_add(1);
            }
        }
        if y + 1 < 10 {
            let yi = (y + 1) as usize;
            if xi < 10 && yi < 10 {
                map[xi][yi] = map[xi][yi].wrapping_add(1);
            }
        }
    }
}

fn taint_3(map: &mut [[i32; 10]; 10], input: &[i32; 3]) {
    let x = input[0];
    let y = input[1];
    if x - 2 >= 0 {
        let xi = (x - 2) as usize;
        let yi = y as usize;
        if xi < 10 && yi < 10 {
            map[xi][yi] = map[xi][yi].wrapping_add(1);
        }
    }
    if x + 2 < 10 {
        let xi = (x + 2) as usize;
        let yi = y as usize;
        if xi < 10 && yi < 10 {
            map[xi][yi] = map[xi][yi].wrapping_add(1);
        }
    }
    if y - 2 >= 0 {
        let xi = x as usize;
        let yi = (y - 2) as usize;
        if xi < 10 && yi < 10 {
            map[xi][yi] = map[xi][yi].wrapping_add(1);
        }
    }
    if y + 2 < 10 {
        let xi = x as usize;
        let yi = (y + 2) as usize;
        if xi < 10 && yi < 10 {
            map[xi][yi] = map[xi][yi].wrapping_add(1);
        }
    }
}

fn untaint(map: &[[i32; 10]; 10]) -> i32 {
    let mut ret: i32 = 0;
    for p in 0..10 {
        for q in 0..10 {
            if map[p][q] == 0 {
                ret = ret.wrapping_add(1);
            }
        }
    }
    ret
}

fn mosttaint(map: &[[i32; 10]; 10]) -> i32 {
    let mut ret: i32 = 0;
    for p in 0..10 {
        for q in 0..10 {
            if map[p][q] > ret {
                ret = map[p][q];
            }
        }
    }
    ret
}
