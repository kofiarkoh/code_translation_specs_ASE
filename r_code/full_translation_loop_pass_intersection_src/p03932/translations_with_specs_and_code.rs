use std::cmp::max;
use std::io::{self, Read};

const D_SIZE_MAX: usize = 205;

fn main() {
    // Read all input as whitespace-separated integers (compatible with C's scanf/fscanf usage).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: usize = it.next().unwrap().parse().unwrap();

    let mut mass = vec![vec![0i32; w]; h];
    for i in 0..h {
        for j in 0..w {
            mass[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    // Fixed-size DP buffer like the C code (unreachable states remain 0).
    let mut cnt = vec![vec![vec![0i32; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX];
    cnt[0][0][0] = mass[0][0];

    // Move along an anti-diagonal: (row--, col++).
    fn move_diag(row: &mut i32, col: &mut i32, w: i32) -> bool {
        *row -= 1;
        *col += 1;
        !(*row < 0 || *col >= w)
    }

    // Add cell values for two positions, counting once if same cell.
    #[inline]
    fn get_add(mass: &[Vec<i32>], r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
        let mut add = mass[r1][c1];
        if r1 == r2 && c1 == c2 {
            return add;
        }
        add += mass[r2][c2];
        add
    }

    let steps = h + w - 2;
    let w_i32 = w as i32;

    // Iterate by synchronized time step i.
    for i in 0..steps {
        // Enumerate all (row1,col1) with row1+col1 == i, starting from the "bottom-most" one.
        let (mut row1, mut col1) = if i < h {
            (i as i32, 0i32)
        } else {
            let r = (h - 1) as i32;
            (r, (i as i32) - r)
        };

        loop {
            let (mut row2, mut col2) = (row1, col1);
            let mut j: usize = 0;

            loop {
                let cur = cnt[row1 as usize][col1 as usize][j];

                // The C code relies on oversized arrays and may compute transitions that are
                // out of the actual grid; in C this is still within the big buffers but would
                // read garbage from si2Mass. In Rust we must avoid OOB; we therefore only
                // apply transitions whose target cells are inside the grid.
                //
                // This matches the intended semantics from the specification: agents must
                // remain within the grid at all times.

                // Down/Down
                if (row1 + 1) < h as i32 && (row2 + 1) < h as i32 {
                    let add = get_add(
                        &mass,
                        (row1 + 1) as usize,
                        col1 as usize,
                        (row2 + 1) as usize,
                        col2 as usize,
                    );
                    let dst = &mut cnt[(row1 + 1) as usize][col1 as usize][j];
                    *dst = max(*dst, cur + add);
                }

                // Down/Right
                if (row1 + 1) < h as i32 && (col2 + 1) < w as i32 {
                    let add = get_add(
                        &mass,
                        (row1 + 1) as usize,
                        col1 as usize,
                        row2 as usize,
                        (col2 + 1) as usize,
                    );
                    let dst = &mut cnt[(row1 + 1) as usize][col1 as usize][j + 1];
                    *dst = max(*dst, cur + add);
                }

                // Right/Down
                if j > 0 && (col1 + 1) < w as i32 && (row2 + 1) < h as i32 {
                    let add = get_add(
                        &mass,
                        row1 as usize,
                        (col1 + 1) as usize,
                        (row2 + 1) as usize,
                        col2 as usize,
                    );
                    let dst = &mut cnt[row1 as usize][(col1 + 1) as usize][j - 1];
                    *dst = max(*dst, cur + add);
                }

                // Right/Right
                if (col1 + 1) < w as i32 && (col2 + 1) < w as i32 {
                    let add = get_add(
                        &mass,
                        row1 as usize,
                        (col1 + 1) as usize,
                        row2 as usize,
                        (col2 + 1) as usize,
                    );
                    let dst = &mut cnt[row1 as usize][(col1 + 1) as usize][j];
                    *dst = max(*dst, cur + add);
                }

                if !move_diag(&mut row2, &mut col2, w_i32) {
                    break;
                }
                j += 1;
            }

            if !move_diag(&mut row1, &mut col1, w_i32) {
                break;
            }
        }
    }

    let ans = cnt[h - 1][w - 1][0];
    print!("{ans}\n");
}
