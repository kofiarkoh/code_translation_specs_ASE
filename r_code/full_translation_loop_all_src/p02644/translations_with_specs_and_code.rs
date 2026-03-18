use std::cmp::min;
use std::collections::VecDeque;
use std::io::{self, Read};

const INF: i32 = 1_001_001_001;

fn main() {
    // Read entire stdin, split by whitespace (matches scanf behavior for ints and %s lines).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let mut sx: isize = it.next().unwrap().parse::<isize>().unwrap() - 1;
    let mut sy: isize = it.next().unwrap().parse::<isize>().unwrap() - 1;
    let mut gx: isize = it.next().unwrap().parse::<isize>().unwrap() - 1;
    let mut gy: isize = it.next().unwrap().parse::<isize>().unwrap() - 1;

    // Grid: passable '.' and obstacles otherwise (per C code check).
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = it.next().unwrap().as_bytes().to_vec();
        // Preconditions say input is valid; we assume row length == w.
        grid.push(row);
    }

    let mut dist = vec![vec![INF; w]; h];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    let (sxu, syu) = (sx as usize, sy as usize);
    dist[sxu][syu] = 0;
    q.push_back((sxu, syu));

    let dx: [isize; 4] = [1, 0, -1, 0];
    let dy: [isize; 4] = [0, 1, 0, -1];

    while let Some((x, y)) = q.pop_front() {
        let curd = dist[x][y];
        for dir in 0..4 {
            for step in 1..=k {
                let nx = x as isize + step as isize * dx[dir];
                let ny = y as isize + step as isize * dy[dir];

                if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                    break;
                }
                let (nxu, nyu) = (nx as usize, ny as usize);

                if grid[nxu][nyu] != b'.' {
                    break;
                }

                // Mirror C condition:
                // proceed only if dist[nx][ny] >= dist[x][y] + 1, else break.
                let nd = curd + 1;
                if dist[nxu][nyu] < nd {
                    break;
                }
                if dist[nxu][nyu] == nd {
                    continue;
                }

                dist[nxu][nyu] = nd;
                q.push_back((nxu, nyu));
            }
        }
    }

    let ans = {
        let d = dist[gx as usize][gy as usize];
        if d == INF { -1 } else { d }
    };
    print!("{ans}\n");
}
