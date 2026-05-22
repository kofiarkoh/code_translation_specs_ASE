use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let x_opt = it.next();
        if x_opt.is_none() {
            // Spec says malformed/short input is undefined; just stop.
            break;
        }
        let x: usize = x_opt.unwrap().parse().unwrap();
        let y: usize = it.next().unwrap().parse().unwrap();

        if x == 0 && y == 0 {
            break;
        }

        // course[row][col], but input is provided column-by-column: for each col, X integers.
        let mut course = vec![vec![0i32; y]; x];
        for col in 0..y {
            for row in 0..x {
                course[row][col] = it.next().unwrap().parse().unwrap();
            }
        }

        // pattern[row][col], with two extra "beyond-end" columns: y and y+1
        let mut pattern = vec![vec![0i64; y + 2]; x];

        // Start positions: any row in column 0 whose cell value is 0 contributes 1.
        for row in 0..x {
            if course[row][0] == 0 {
                pattern[row][0] = 1;
            }
        }

        // Transitions from column j to j+1 for j in [0, y-2]
        // (If y < 2, this loop does nothing, matching the C behavior.)
        if y >= 2 {
            for j in 0..(y - 1) {
                for i in 0..x {
                    let cur = pattern[i][j];
                    if cur == 0 {
                        continue;
                    }
                    // Attempt moves to rows i-1, i, i+1 in next column.
                    for dk in [-1isize, 0, 1] {
                        let k_isize = i as isize + dk;
                        if k_isize < 0 || k_isize >= x as isize {
                            continue;
                        }
                        let k = k_isize as usize;
                        match course[k][j + 1] {
                            0 => {
                                pattern[k][j + 1] += cur;
                            }
                            1 => {
                                // Blocked: not enterable; clear any accumulated count there.
                                pattern[k][j + 1] = 0;
                            }
                            _jump => {
                                // Spec: any value other than 0/1 is a jump cell.
                                // Only allowed when moving straight (same row).
                                if k == i {
                                    // Base landing column is j+3.
                                    // While landing cell is also a jump cell and landing column is
                                    // still strictly before the last column, advance by 2.
                                    //
                                    // NOTE: The provided C code only treats value==2 as "jump"
                                    // during the while-scan. The spec says "any other value".
                                    // We follow the spec here.
                                    let mut landing = j + 3;
                                    while landing < y {
                                        let v = course[k][landing];
                                        if v != 0 && v != 1 && landing < y - 1 {
                                            landing += 2;
                                        } else {
                                            break;
                                        }
                                    }
                                    // landing may be y or y+1 (beyond-end), which are valid in pattern.
                                    if landing < y + 2 {
                                        pattern[k][landing] += cur;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sum counts in last column and the two beyond-end columns.
        let mut sum: i64 = 0;
        for row in 0..x {
            for col in (y - 1)..=(y + 1) {
                sum += pattern[row][col];
            }
        }

        println!("{sum}");
    }
}
