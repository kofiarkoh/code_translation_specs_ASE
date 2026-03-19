use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // 1-cell padding to represent "outside" as reachable empty space.
    let ph = h + 2;
    let pw = w + 2;

    let mut cell = vec![vec![0i32; pw]; ph];
    for y in 0..h {
        for x in 0..w {
            let v: i32 = match it.next() {
                Some(s) => s.parse().unwrap(),
                None => return,
            };
            cell[y + 1][x + 1] = v;
        }
    }

    // Neighbor offsets for the common "odd-r" horizontal layout:
    // parity is based on the original row index (0-based). Since we padded by 1,
    // the original row y corresponds to padded row (y+1), so parity must be taken
    // from (padded_y - 1), i.e., (padded_y + 1) & 1.
    const DY: [isize; 6] = [-1, -1, 0, 0, 1, 1];
    const DX_EVEN: [isize; 6] = [-1, 0, -1, 1, -1, 0]; // original row even
    const DX_ODD: [isize; 6] = [0, 1, -1, 1, 0, 1]; // original row odd

    let mut visited = vec![vec![false; pw]; ph];
    let mut q = VecDeque::new();
    visited[0][0] = true;
    q.push_back((0usize, 0usize));

    while let Some((y, x)) = q.pop_front() {
        let dxs = if ((y + 1) & 1) == 0 { &DX_EVEN } else { &DX_ODD };
        for k in 0..6 {
            let ny = y as isize + DY[k];
            let nx = x as isize + dxs[k];
            if ny < 0 || nx < 0 {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if ny >= ph || nx >= pw {
                continue;
            }
            if visited[ny][nx] {
                continue;
            }
            if cell[ny][nx] != 0 {
                continue; // non-empty blocks traversal
            }
            visited[ny][nx] = true;
            q.push_back((ny, nx));
        }
    }

    let mut perimeter: i64 = 0;
    for y in 0..ph {
        let dxs = if ((y + 1) & 1) == 0 { &DX_EVEN } else { &DX_ODD };
        for x in 0..pw {
            if cell[y][x] != 1 {
                continue; // only value 1 counts as filled for perimeter
            }
            for k in 0..6 {
                let ny = y as isize + DY[k];
                let nx = x as isize + dxs[k];
                if ny < 0 || nx < 0 {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                if ny >= ph || nx >= pw {
                    continue;
                }
                if visited[ny][nx] {
                    perimeter += 1;
                }
            }
        }
    }

    print!("{}", perimeter);
}
