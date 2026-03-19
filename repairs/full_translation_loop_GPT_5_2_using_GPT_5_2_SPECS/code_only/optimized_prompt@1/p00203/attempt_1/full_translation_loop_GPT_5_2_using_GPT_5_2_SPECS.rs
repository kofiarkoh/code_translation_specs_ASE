use std::io::{self, Read};

fn main() {
    // Read entire stdin; the C code uses fgets/strtok/atoi line-by-line, but behavior here
    // is equivalent for all well-formed inputs the C code can process.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    while ski_resort(&mut it) {}
}

fn ski_resort<'a, I>(it: &mut I) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let (x, y) = listen_size(it);
    if x == 0 && y == 0 {
        return false;
    }

    // course[X][Y] in C, but filled by columns (j outer, i inner).
    let mut course = vec![vec![0i32; y]; x];
    listen_course(it, &mut course, x, y);
    analyze_course(&course, x, y);

    true
}

/// Mimics `listen`: repeatedly reads a line until it is not exactly "\n".
/// With `lines()`, blank lines become "" (no '\n'), so we treat "" as the blank line.
fn listen_nonblank_line<'a, I>(it: &mut I) -> String
where
    I: Iterator<Item = &'a str>,
{
    loop {
        let line = match it.next() {
            Some(s) => s,
            None => "", // EOF: C's fgets would return NULL; original code would UB. We degrade to "".
        };
        if line != "" {
            return line.to_string();
        }
    }
}

/// C's `atoi` behavior: parses optional sign and digits; stops at first non-digit.
/// If no conversion, returns 0.
fn atoi_like(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut i = 0;

    // Skip leading whitespace (C's atoi does).
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    let mut sign: i32 = 1;
    if i < bytes.len() {
        if bytes[i] == b'+' {
            i += 1;
        } else if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        }
    }

    let mut any = false;
    let mut val: i32 = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        any = true;
        let d = (bytes[i] - b'0') as i32;
        // C's atoi has undefined behavior on overflow; we choose wrapping to emulate typical behavior.
        val = val.wrapping_mul(10).wrapping_add(d);
        i += 1;
    }

    if !any {
        0
    } else {
        val.wrapping_mul(sign)
    }
}

fn listen_size<'a, I>(it: &mut I) -> (usize, usize)
where
    I: Iterator<Item = &'a str>,
{
    let line = listen_nonblank_line(it);
    // C uses strtok with delimiter " " (space only), not general whitespace.
    // We'll split on spaces and ignore empty fields to approximate strtok's behavior.
    let mut parts = line.split(' ').filter(|p| !p.is_empty());
    let x = parts.next().map(atoi_like).unwrap_or(0);
    let y = parts.next().map(atoi_like).unwrap_or(0);

    // C stores into int; negative sizes would be UB for VLA. We map negatives to 0 to avoid panic.
    let xu = if x > 0 { x as usize } else { 0 };
    let yu = if y > 0 { y as usize } else { 0 };
    (xu, yu)
}

fn listen_course<'a, I>(it: &mut I, course: &mut [Vec<i32>], x: usize, y: usize)
where
    I: Iterator<Item = &'a str>,
{
    for j in 0..y {
        let line = listen_nonblank_line(it);
        let mut parts = line.split(' ').filter(|p| !p.is_empty());

        // course[0][j] from first token, then course[i][j] from subsequent tokens.
        let first = parts.next().map(atoi_like).unwrap_or(0);
        if x > 0 {
            course[0][j] = first;
        }
        for i in 1..x {
            let v = parts.next().map(atoi_like).unwrap_or(0);
            course[i][j] = v;
        }
    }
}

fn analyze_course(course: &[Vec<i32>], x: usize, y: usize) {
    // pattern[X][Y+2] initialized to 0.
    let mut pattern = vec![vec![0i32; y + 2]; x];

    for i in 0..x {
        if y > 0 && course[i][0] == 0 {
            pattern[i][0] = 1;
        }
    }

    if y >= 2 {
        for j in 0..(y - 1) {
            for i in 0..x {
                // k from i-1 to i+1 inclusive
                let i_i32 = i as i32;
                for k_i32 in (i_i32 - 1)..=(i_i32 + 1) {
                    if k_i32 < 0 || k_i32 >= x as i32 {
                        continue;
                    }
                    let k = k_i32 as usize;
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
                                let idx = j + 3 + m; // may be y or y+1 (fits in pattern's Y+2)
                                if idx < y + 2 {
                                    pattern[k][idx] = pattern[k][idx].wrapping_add(add);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut sum: i32 = 0;
    for i in 0..x {
        // k from Y-1 to Y+1 inclusive
        if y == 0 {
            // In C, Y==0 would make Y-1 underflow and loops nonsensical; but VLA would be 0.
            // We keep sum as 0.
            continue;
        }
        for k in (y - 1)..=(y + 1) {
            sum = sum.wrapping_add(pattern[i][k]);
        }
    }

    print!("{sum}\n");
}
