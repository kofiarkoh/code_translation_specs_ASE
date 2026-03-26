use std::io::{self, BufRead};

const MAX: usize = 1005;

fn read_input() -> (usize, Vec<Vec<u8>>) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut grid = vec![vec![0; n]; n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            if c == '1' {
                grid[j][i] = 1;
            }
        }
    }
    
    (n, grid)
}

fn bipartite_matching(n: usize, to: &Vec<Vec<usize>>, hi: &mut Vec<usize>, match_arr: &mut Vec<i32>) -> usize {
    let mut max = 0;
    let mut seen = vec![0; n];
    let mut mark = 1;

    for u in 0..n {
        if match_arr[u] < 0 {
            mark += 1;
            if bpm(u, &to, hi, match_arr, &mut seen, mark) {
                max += 1;
            }
        }
    }
    max
}

fn bpm(u: usize, to: &Vec<Vec<usize>>, hi: &Vec<usize>, match_arr: &mut Vec<i32>, seen: &mut Vec<i32>, mark: i32) -> bool {
    seen[u] = mark;
    for &v in &to[u][..hi[u]] {
        let w = match_arr[v];
        if w < 0 || (seen[w as usize] != mark && bpm(w as usize, to, hi, match_arr, seen, mark)) {
            match_arr[v] = u as i32;
            match_arr[u] = v as i32;
            return true;
        }
    }
    false
}

fn main() {
    let (n, grid) = read_input();
    let mut hi = vec![0; n];
    let mut to = vec![vec![]; n];
    let mut match_arr = vec![-1; n + n];

    for i in 0..n {
        for j in 0..n {
            if grid[j][i] == 1 {
                to[j].push(n + i);
                hi[j] += 1;
            }
        }
    }

    if bipartite_matching(n, &to, &mut hi, &mut match_arr) < n {
        println!("no");
    } else {
        println!("yes");
        for i in 0..n {
            match_arr[match_arr[i] as usize] = -1;
            match_arr[i] = -1;
            let mut seen = vec![0; n];
            let mut mark = 1;
            bpm(i, &to, &hi, &mut match_arr, &mut seen, mark);
            hi[i] = 0;
            to[i].push(match_arr[i] as usize);
            // Convert match_arr[i] to usize for correct indexing
            println!("{}", match_arr[i] as usize - n + 1);
        }
    }
}
