use std::cmp::max;
use std::io::{self, Read};

const D_SIZE_MAX: usize = 205;

fn get_add(
    mass: &[[i32; D_SIZE_MAX]; D_SIZE_MAX],
    row: usize,
    col: usize,
    row2: usize,
    col2: usize,
) -> i32 {
    let mut add = mass[row][col];
    if row == row2 && col == col2 {
        return add;
    }
    add += mass[row2][col2];
    add
}

fn main() {
    // Read entire stdin (matches typical scanf/fgets behavior for contest inputs).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Width/Height are read as: H then W (per sscanf("%d%d", &siH, &siW))
    let si_h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // no input
    };
    let si_w: usize = it.next().unwrap().parse().unwrap();

    // Mass grid (only [0..H)[0..W) is used)
    let mut mass = [[0i32; D_SIZE_MAX]; D_SIZE_MAX];
    for i in 0..si_h {
        for j in 0..si_w {
            mass[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    // si3Cnt[H][W][2*D_SIZE_MAX] (only indices used by the C code are touched)
    let mut cnt = vec![vec![vec![0i32; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX];

    // Initial value
    cnt[0][0][0] = mass[0][0];

    // Loop by number of moves
    for step in 0..(si_w + si_h - 2) {
        // First person's starting position on this anti-diagonal
        let (mut row1, mut col1) = if step < si_h {
            (step, 0usize)
        } else {
            let r = si_h - 1;
            (r, step - r)
        };

        loop {
            // Second person's position loop (moves up-right along the same anti-diagonal)
            let mut row2 = row1;
            let mut col2 = col1;

            for j in 0usize.. {
                // The C code assumes these indices are valid for the given input constraints.
                // We keep the same indexing behavior (panic on invalid input rather than
                // silently changing semantics).

                // down, down
                let add = get_add(&mass, row1 + 1, col1, row2 + 1, col2);
                cnt[row1 + 1][col1][j] = max(cnt[row1 + 1][col1][j], cnt[row1][col1][j] + add);

                // down, right
                let add = get_add(&mass, row1 + 1, col1, row2, col2 + 1);
                cnt[row1 + 1][col1][j + 1] =
                    max(cnt[row1 + 1][col1][j + 1], cnt[row1][col1][j] + add);

                // right, down
                if j > 0 {
                    let add = get_add(&mass, row1, col1 + 1, row2 + 1, col2);
                    cnt[row1][col1 + 1][j - 1] =
                        max(cnt[row1][col1 + 1][j - 1], cnt[row1][col1][j] + add);
                }

                // right, right
                let add = get_add(&mass, row1, col1 + 1, row2, col2 + 1);
                cnt[row1][col1 + 1][j] =
                    max(cnt[row1][col1 + 1][j], cnt[row1][col1][j] + add);

                // Move second person up-right; stop if out of bounds (matches fMove)
                if row2 == 0 || col2 + 1 >= si_w {
                    break;
                }
                row2 -= 1;
                col2 += 1;
            }

            // Move first person up-right; stop if out of bounds (matches fMove)
            if row1 == 0 || col1 + 1 >= si_w {
                break;
            }
            row1 -= 1;
            col1 += 1;
        }
    }

    let ans = cnt[si_h - 1][si_w - 1][0];
    print!("{ans}\n");
}
