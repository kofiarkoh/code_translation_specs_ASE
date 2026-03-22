use std::io::{self, Read};

fn main() {
    // Read entire stdin; the C code uses fgets/strtok/atoi line-by-line, but behavior here
    // is equivalent for all well-formed inputs and preserves output formatting.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    while ski_resort(&mut it) {}
}

fn ski_resort<'a, I>(it: &mut I) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let x = match it.next() {
        None => return false, // no more input
        Some(s) => atoi_like_c(s),
    };
    let y = match it.next() {
        None => return false,
        Some(s) => atoi_like_c(s),
    };

    if x == 0 && y == 0 {
        return false;
    }

    // In C, X and Y are ints and used for VLA sizes; negative would be UB.
    // We mirror the "int" arithmetic and then require non-negative sizes to allocate.
    let x_usize = if x > 0 { x as usize } else { 0 };
    let y_usize = if y > 0 { y as usize } else { 0 };

    // course[X][Y] stored column-wise in input: for each j in 0..Y, read X ints.
    let mut course = vec![vec![0i32; y_usize]; x_usize];
    for j in 0..y_usize {
        for i in 0..x_usize {
            let v = it.next().map(atoi_like_c).unwrap_or(0);
            course[i][j] = v;
        }
    }

    analyze_course(&course, x_usize, y_usize);
    true
}

fn analyze_course(course: &Vec<Vec<i32>>, x: usize, y: usize) {
    // pattern[X][Y+2] initialized to 0
    let mut pattern = vec![vec![0i32; y + 2]; x];

    // for i: if course[i][0]==0 then pattern[i][0]=1
    if y > 0 {
        for i in 0..x {
            if course[i][0] == 0 {
                pattern[i][0] = 1;
            }
        }
    }

    // for j=0; j<Y-1; j++
    // Note: in C, if Y<=0 then loop doesn't run; if Y==0, Y-1 is -1 and condition false.
    if y >= 2 {
        for j in 0..(y - 1) {
            for i in 0..x {
                // for k=i-1; k<=i+1; k++
                for k_i32 in (i as i32 - 1)..=(i as i32 + 1) {
                    if k_i32 < 0 || k_i32 >= x as i32 {
                        continue;
                    }
                    let k = k_i32 as usize;
                    match course[k][j + 1] {
                        0 => {
                            // pattern[k][j+1] += pattern[i][j]
                            pattern[k][j + 1] = pattern[k][j + 1].wrapping_add(pattern[i][j]);
                        }
                        1 => {
                            // pattern[k][j+1] = 0
                            pattern[k][j + 1] = 0;
                        }
                        _ => {
                            if k == i {
                                // m=0; while(j+3+m<Y) { if course[k][j+3+m]==2 m+=2 else break }
                                let mut m: usize = 0;
                                while j + 3 + m < y {
                                    if course[k][j + 3 + m] == 2 {
                                        m += 2;
                                    } else {
                                        break;
                                    }
                                }
                                // pattern[k][j+3+m] += pattern[i][j]
                                let idx = j + 3 + m;
                                if idx < y + 2 {
                                    pattern[k][idx] =
                                        pattern[k][idx].wrapping_add(pattern[i][j]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // sum over i and k=Y-1..=Y+1 of pattern[i][k]
    // In C, if Y==0 then k starts at -1 and reads out of bounds (UB).
    // We avoid UB by only summing valid indices; for typical valid inputs (Y>=1),
    // this matches the C behavior.
    let mut sum: i32 = 0;
    if y >= 1 {
        for i in 0..x {
            for k in (y - 1)..=(y + 1) {
                sum = sum.wrapping_add(pattern[i][k]);
            }
        }
    }

    print!("{sum}\n");
}

/// Approximate C's `atoi`:
/// - parses optional leading sign and consecutive digits
/// - stops at first non-digit
/// - returns 0 if no digits
/// - uses wrapping i32 arithmetic to emulate overflow behavior (C is UB, but this is a best-effort)
fn atoi_like_c(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut i = 0usize;

    // C's atoi skips leading whitespace, but our tokenization already did that.
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
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_digit() {
            any = true;
            let d = (b - b'0') as i32;
            val = val.wrapping_mul(10).wrapping_add(d);
            i += 1;
        } else {
            break;
        }
    }

    if !any {
        0
    } else {
        val.wrapping_mul(sign)
    }
}
