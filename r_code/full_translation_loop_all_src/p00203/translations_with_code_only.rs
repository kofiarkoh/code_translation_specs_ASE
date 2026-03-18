use std::io::{self, Read};

fn main() {
    // Read entire stdin; the C code uses fgets/strtok/atoi line-by-line, but behavior here
    // is equivalent for all well-formed inputs and preserves whitespace/newline output.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    while ski_resort(&mut it) {}
}

fn ski_resort<'a, I: Iterator<Item = &'a str>>(it: &mut I) -> bool {
    let x = match it.next() {
        Some(s) => atoi_like_c(s),
        None => return false, // no more input
    };
    let y = match it.next() {
        Some(s) => atoi_like_c(s),
        None => return false,
    };

    if x == 0 && y == 0 {
        return false;
    }

    // C uses VLA int course[X][Y] and then an array of row pointers.
    // We'll store as Vec<Vec<i32>> with indexing [i][j] matching course[i][j].
    let x_usize = if x < 0 { 0 } else { x as usize };
    let y_usize = if y < 0 { 0 } else { y as usize };

    let mut course = vec![vec![0i32; y_usize]; x_usize];
    listen_course(&mut course, x_usize, y_usize, it);
    analyze_course(&course, x_usize, y_usize);

    true
}

fn listen_course<'a, I: Iterator<Item = &'a str>>(
    course: &mut [Vec<i32>],
    x: usize,
    y: usize,
    it: &mut I,
) {
    // C reads Y lines; each line has X integers, assigned course[i][j] for i=0..X-1.
    for j in 0..y {
        for i in 0..x {
            let tok = it.next().unwrap_or("0");
            course[i][j] = atoi_like_c(tok);
        }
    }
}

fn analyze_course(course: &[Vec<i32>], x: usize, y: usize) {
    // pattern[X][Y+2] initialized to 0
    let mut pattern = vec![vec![0i32; y + 2]; x];

    // Initialize first column based on course[i][0] == 0
    if y > 0 {
        for i in 0..x {
            if course[i][0] == 0 {
                pattern[i][0] = 1;
            }
        }
    }

    // Main DP
    if y >= 2 {
        for j in 0..(y - 1) {
            for i in 0..x {
                // k from i-1 to i+1 inclusive, with bounds checks
                let k_start = i.saturating_sub(1);
                let k_end = (i + 1).min(x.saturating_sub(1));
                for k in k_start..=k_end {
                    match course[k][j + 1] {
                        0 => {
                            let add = pattern[i][j];
                            pattern[k][j + 1] = pattern[k][j + 1].wrapping_add(add);
                        }
                        1 => {
                            pattern[k][j + 1] = 0;
                        }
                        _ => {
                            if k == i {
                                let mut m: usize = 0;
                                while j + 3 + m < y {
                                    if course[k][j + 3 + m] == 2 {
                                        m += 2;
                                    } else {
                                        break;
                                    }
                                }
                                let add = pattern[i][j];
                                let idx = j + 3 + m; // guaranteed < y+2
                                pattern[k][idx] = pattern[k][idx].wrapping_add(add);
                            }
                        }
                    }
                }
            }
        }
    }

    // Sum pattern[i][Y-1..=Y+1]
    let mut sum: i32 = 0;
    if y > 0 {
        for i in 0..x {
            for k in (y - 1)..=(y + 1) {
                sum = sum.wrapping_add(pattern[i][k]);
            }
        }
    } else {
        // If Y==0, the C code would have undefined behavior (accessing Y-1).
        // We choose a conservative behavior: sum remains 0.
    }

    print!("{sum}\n");
}

/// Approximate C's atoi behavior:
/// - Parses optional leading sign and consecutive decimal digits.
/// - Stops at first non-digit.
/// - If no digits, returns 0.
/// - Overflow in C is undefined; we saturate to i32 bounds to avoid panics.
fn atoi_like_c(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut i = 0usize;

    // Optional sign
    let mut sign: i64 = 1;
    if i < bytes.len() {
        if bytes[i] == b'+' {
            i += 1;
        } else if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        }
    }

    let mut acc: i64 = 0;
    let mut any = false;
    while i < bytes.len() {
        let b = bytes[i];
        if (b'0'..=b'9').contains(&b) {
            any = true;
            acc = acc * 10 + (b - b'0') as i64;
            // clamp to avoid i64 runaway; final clamp to i32 below
            if acc > (i32::MAX as i64) + 1 {
                acc = (i32::MAX as i64) + 1;
            }
            i += 1;
        } else {
            break;
        }
    }

    if !any {
        return 0;
    }

    let val = acc * sign;
    if val > i32::MAX as i64 {
        i32::MAX
    } else if val < i32::MIN as i64 {
        i32::MIN
    } else {
        val as i32
    }
}
