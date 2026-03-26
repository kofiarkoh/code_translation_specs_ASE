use std::io::{self, BufRead};

const MAX_SIZE: usize = 128;

fn dfs(i: usize, j: usize, h: usize, w: usize, str: &Vec<Vec<char>>, area_id: &mut Vec<Vec<usize>>) {
    // Check down
    if i + 1 < h {
        if str[i][j] == str[i + 1][j] && area_id[i + 1][j] == (i + 1) * w + j {
            area_id[i + 1][j] = area_id[i][j];
            dfs(i + 1, j, h, w, str, area_id);
        }
    }

    // Check up
    if i > 0 {
        if str[i][j] == str[i - 1][j] && area_id[i - 1][j] == (i - 1) * w + j {
            area_id[i - 1][j] = area_id[i][j];
            dfs(i - 1, j, h, w, str, area_id);
        }
    }

    // Check right
    if j + 1 < w {
        if str[i][j] == str[i][j + 1] && area_id[i][j + 1] == i * w + (j + 1) {
            area_id[i][j + 1] = area_id[i][j];
            dfs(i, j + 1, h, w, str, area_id);
        }
    }

    // Check left
    if j > 0 {
        if str[i][j] == str[i][j - 1] && area_id[i][j - 1] == i * w + (j - 1) {
            area_id[i][j - 1] = area_id[i][j];
            dfs(i, j - 1, h, w, str, area_id);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut area_num = vec![0; MAX_SIZE * MAX_SIZE];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut dimensions = line.split_whitespace();
        let h: usize = dimensions.next().unwrap().parse().unwrap();
        let w: usize = dimensions.next().unwrap().parse().unwrap();

        if h == 0 && w == 0 {
            break;
        }

        let mut str = vec![vec![' '; MAX_SIZE]; MAX_SIZE];
        let mut area_id = vec![vec![0; MAX_SIZE]; MAX_SIZE];

        for i in 0..h {
            area_id[i] = (0..w).map(|j| i * w + j).collect();
        }

        area_num.fill(0);

        for i in 0..h {
            let input_line = reader.lines().next().unwrap().unwrap();
            str[i] = input_line.chars().collect();
        }

        for i in 0..h {
            for j in 0..w {
                if area_id[i][j] == i * w + j { // Only start DFS if it's the original position
                    dfs(i, j, h, w, &str, &mut area_id);
                }
            }
        }

        for i in 0..h {
            for j in 0..w {
                area_num[area_id[i][j]] = 1;
            }
        }

        let cnt = area_num.iter().filter(|&&x| x == 1).count();
        println!("{}", cnt);
    }
}
