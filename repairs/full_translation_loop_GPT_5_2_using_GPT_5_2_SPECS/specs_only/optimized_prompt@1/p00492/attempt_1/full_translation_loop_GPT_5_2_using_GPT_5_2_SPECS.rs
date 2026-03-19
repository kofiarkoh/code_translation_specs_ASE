use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // Undefined by spec; choose to do nothing if no input.
    };
    let h: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Pad the grid with a 1-cell border so "outside" is representable as (0,0).
    // This matches the spec's notion of exterior reachable from outside bounds.
    let ph = h + 2;
    let pw = w + 2;

    // cell[y][x] stores the original value (0 empty, 1 filled, other treated as non-empty for traversal).
    let mut cell = vec![vec![0i32; pw]; ph];
    for y in 0..h {
        for x in 0..w {
            if let Some(v) = it.next() {
                cell[y + 1][x + 1] = v.parse().unwrap();
            } else {
                // Undefined by spec; stop early.
                return;
            }
        }
    }

    // visited marks exterior-reachable empty positions (value == 0).
    let mut visited = vec![vec![false; pw]; ph];
    let mut q = VecDeque::new();

    // Start BFS from the padded outside corner, which is guaranteed empty (initialized to 0).
    visited[0][0] = true;
    q.push_back((0usize, 0usize));

    // Hex neighbor offsets depend on row parity.
    // We use y parity in the padded grid; this preserves the intended parity-based adjacency.
    // For even rows:
    //   (-1, 0), (-1, +1), (0, -1), (0, +1), (+1, 0), (+1, +1)
    // For odd rows:
    //   (-1, -1), (-1, 0), (0, -1), (0, +1), (+1, -1), (+1, 0)
    const DY: [isize; 6] = [-1, -1, 0, 0, 1, 1];
    const DX_EVEN: [isize; 6] = [0, 1, -1, 1, 0, 1];
    const DX_ODD: [isize; 6] = [-1, 0, -1, 1, -1, 0];

    // BFS through empty cells only (value == 0). Non-zero blocks traversal.
    while let Some((y, x)) = q.pop_front() {
        let dxs = if (y & 1) == 0 { &DX_EVEN } else { &DX_ODD };
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
                continue; // blocks traversal
            }
            visited[ny][nx] = true;
            q.push_back((ny, nx));
        }
    }

    // Count perimeter edges: for each filled cell (value == 1 only),
    // count neighbor edges adjacent to exterior-reachable empty cells (visited == true).
    // Other non-zero values are "not filled" for perimeter counting per spec.
    let mut perimeter: i64 = 0;
    for y in 0..ph {
        let dxs = if (y & 1) == 0 { &DX_EVEN } else { &DX_ODD };
        for x in 0..pw {
            if cell[y][x] != 1 {
                continue;
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
                    // visited implies empty and connected to exterior.
                    perimeter += 1;
                }
            }
        }
    }

    print!("{}", perimeter);
}
