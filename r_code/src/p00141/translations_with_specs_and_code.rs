use std::io::{self, BufRead};

const U: usize = 1;
const R: usize = 2;
const D: usize = 3;
const L: usize = 4;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let cn: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for ci in 0..cn {
        // Read the size of the spiral
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut a = [[' '; 105]; 105];

        // Initialize the grid with '#' borders
        for r in (0..n + 4).step_by(n + 3) {
            for c in 0..n + 4 {
                a[r][c] = '#';
            }
        }
        for c in (0..n + 4).step_by(n + 3) {
            for r in 0..n + 4 {
                a[r][c] = '#';
            }
        }

        let mut r = n + 1;
        let mut c = 2;
        let mut d = U;
        let mut stop = 0;
        a[r][c] = '#';

        // Construct the spiral
        loop {
            if stop >= 4 {
                break;
            }
            match d {
                U => {
                    if a[r - 2][c] == '#' || a[r - 1][c] == '#' || a[r - 1][c + 1] == '#' {
                        d = R;
                        stop += 1;
                    } else {
                        r -= 1;
                        a[r][c] = '#';
                        stop = 0;
                    }
                }
                R => {
                    if a[r][c + 2] == '#' || a[r][c + 1] == '#' || a[r + 1][c + 1] == '#' {
                        d = D;
                        stop += 1;
                    } else {
                        c += 1;
                        a[r][c] = '#';
                        stop = 0;
                    }
                }
                D => {
                    if a[r + 2][c] == '#' || a[r + 1][c] == '#' || a[r + 1][c - 1] == '#' {
                        d = L;
                        stop += 1;
                    } else {
                        r += 1;
                        a[r][c] = '#';
                        stop = 0;
                    }
                }
                L => {
                    if a[r][c - 2] == '#' || a[r][c - 1] == '#' || a[r - 1][c - 1] == '#' {
                        d = U;
                        stop += 1;
                    } else {
                        c -= 1;
                        a[r][c] = '#';
                        stop = 0;
                    }
                }
                _ => unreachable!(),
            }
        }

        // Print the spiral
        if ci > 0 {
            println!();
        }
        for r in 2..n + 2 {
            let row: String = a[r][2..n + 2].iter().collect();
            println!("{}", row);
        }
    }
}
