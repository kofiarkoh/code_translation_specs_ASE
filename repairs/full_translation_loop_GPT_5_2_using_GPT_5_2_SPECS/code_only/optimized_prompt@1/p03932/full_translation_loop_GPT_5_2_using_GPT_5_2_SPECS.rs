use std::cmp::max;
use std::io::{self, Read};

const D_SIZE_MAX: usize = 205;
const D_DIST_MAX: usize = D_SIZE_MAX * 2;

fn get_add(mass: &[[i32; D_SIZE_MAX]; D_SIZE_MAX], r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
    let mut add = mass[r1][c1];
    if r1 == r2 && c1 == c2 {
        return add;
    }
    add += mass[r2][c2];
    add
}

// C fMove: row--, col++, then check row < 0 or col >= siW.
fn move_up_right(row: &mut i32, col: &mut i32, w: i32) -> i32 {
    *row -= 1;
    *col += 1;
    if *row < 0 {
        return -1;
    }
    if *col >= w {
        return -1;
    }
    0
}

fn main() {
    // Read all input then parse as tokens (equivalent for this program's usage).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let si_h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let si_w: usize = it.next().unwrap().parse().unwrap();

    // Heap-allocate the large arrays to avoid stack overflow.
    // This preserves the C "static storage" behavior (zero-initialized).
    let mut mass: Box<[[i32; D_SIZE_MAX]; D_SIZE_MAX]> = Box::new([[0; D_SIZE_MAX]; D_SIZE_MAX]);
    let mut cnt: Box<[[[i32; D_DIST_MAX]; D_SIZE_MAX]; D_SIZE_MAX]> =
        Box::new([[[0; D_DIST_MAX]; D_SIZE_MAX]; D_SIZE_MAX]);

    for i in 0..si_h {
        for j in 0..si_w {
            mass[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    cnt[0][0][0] = mass[0][0];

    let total_moves = si_w + si_h - 2;
    for i in 0..total_moves {
        // First person's starting position for this move count.
        let (mut row1, mut col1): (i32, i32) = if i < si_h {
            (i as i32, 0)
        } else {
            let r = (si_h - 1) as i32;
            (r, (i as i32) - r)
        };

        loop {
            // Second person's position loop
            let mut row2 = row1;
            let mut col2 = col1;

            let mut j: usize = 0;
            loop {
                let r1u = row1 as usize;
                let c1u = col1 as usize;
                let r2u = row2 as usize;
                let c2u = col2 as usize;

                // The C code relies on the fact that these indices stay within the
                // statically allocated [205][205][410] buffers. We mirror that by
                // keeping the same fixed maximum sizes and not shrinking to si_h/si_w.
                // (So even when r1u+1 == si_h, it is still within D_SIZE_MAX.)
                // down, down
                {
                    let add = get_add(&mass, r1u + 1, c1u, r2u + 1, c2u);
                    let v = cnt[r1u][c1u][j] + add;
                    cnt[r1u + 1][c1u][j] = max(cnt[r1u + 1][c1u][j], v);
                }

                // down, right
                {
                    let add = get_add(&mass, r1u + 1, c1u, r2u, c2u + 1);
                    let v = cnt[r1u][c1u][j] + add;
                    cnt[r1u + 1][c1u][j + 1] = max(cnt[r1u + 1][c1u][j + 1], v);
                }

                // right, down
                if j > 0 {
                    let add = get_add(&mass, r1u, c1u + 1, r2u + 1, c2u);
                    let v = cnt[r1u][c1u][j] + add;
                    cnt[r1u][c1u + 1][j - 1] = max(cnt[r1u][c1u + 1][j - 1], v);
                }

                // right, right
                {
                    let add = get_add(&mass, r1u, c1u + 1, r2u, c2u + 1);
                    let v = cnt[r1u][c1u][j] + add;
                    cnt[r1u][c1u + 1][j] = max(cnt[r1u][c1u + 1][j], v);
                }

                // Next position for second person
                if move_up_right(&mut row2, &mut col2, si_w as i32) != 0 {
                    break;
                }
                j += 1;
            }

            // Next position for first person
            if move_up_right(&mut row1, &mut col1, si_w as i32) != 0 {
                break;
            }
        }
    }

    let ans = cnt[si_h - 1][si_w - 1][0];
    print!("{ans}\n");
}
