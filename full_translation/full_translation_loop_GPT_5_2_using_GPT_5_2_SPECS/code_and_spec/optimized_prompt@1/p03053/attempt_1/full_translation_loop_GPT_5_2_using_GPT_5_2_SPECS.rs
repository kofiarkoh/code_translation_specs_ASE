use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, Read};

const INF: i32 = 1_001_001_001;

fn main() {
    // Read entire stdin, split by whitespace (matches scanf("%d%d") and scanf("%s") behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();

    // Grid and distance array.
    let mut a: Vec<Vec<u8>> = vec![vec![0u8; w]; h];
    let mut b: Vec<Vec<i32>> = vec![vec![INF; w]; h];

    // Multi-source BFS queue.
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..h {
        let row = it.next().unwrap().as_bytes().to_vec();
        // Spec precondition: each row has exactly width characters.
        for j in 0..w {
            a[i][j] = row[j];
            if a[i][j] == b'#' {
                b[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }

    // BFS over 4-neighborhood.
    let dx: [isize; 4] = [0, 1, 0, -1];
    let dy: [isize; 4] = [1, 0, -1, 0];

    while let Some((x, y)) = q.pop_front() {
        let cur = b[x][y];
        for k in 0..4 {
            let nx = x as isize + dx[k];
            let ny = y as isize + dy[k];
            if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                continue;
            }
            let nxu = nx as usize;
            let nyu = ny as usize;

            // C code only relaxes into '.' cells; preserve that behavior.
            if a[nxu][nyu] == b'.' && b[nxu][nyu] > cur + 1 {
                b[nxu][nyu] = cur + 1;
                q.push_back((nxu, nyu));
            }
        }
    }

    // Compute maximum distance.
    let mut ans: i32 = 0;
    for i in 0..h {
        for j in 0..w {
            ans = max(ans, b[i][j]);
        }
    }

    print!("{ans}\n");
}
