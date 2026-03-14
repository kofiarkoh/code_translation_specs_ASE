use std::io::{self, BufRead};

const MAX_N: usize = 200;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut circle = [[0.0; 2]; MAX_N];
        let mut flag = [[0; MAX_N]; MAX_N];
        let mut overlap = [[0; MAX_N]; MAX_N];
        let mut num = [0; MAX_N];
        let mut max_overlap = 1;

        // Read circle coordinates
        for i in 0..n {
            let coords: Vec<&str> = reader.lines().next().unwrap().unwrap().split(',').collect();
            circle[i][0] = coords[0].trim().parse().unwrap();
            circle[i][1] = coords[1].trim().parse().unwrap();
        }

        // Determine overlaps
        for i in 0..n {
            for j in i..n {
                if i == j {
                    flag[i][j] = 1;
                } else {
                    let d = (circle[i][0] - circle[j][0]).powi(2) + (circle[i][1] - circle[j][1]).powi(2);
                    if d <= 4.0 {
                        flag[i][j] = 1;
                        flag[j][i] = 1;
                    }
                }
            }
        }

        // Calculate maximum overlaps
        loop {
            for i in 0..n {
                num[i] = (0..n).filter(|&j| flag[i][j] == 1).count();
            }

            let mut cont_flag = false;
            for i in 0..n {
                if num[i] == 2 {
                    max_overlap = 2;
                }
                if num[i] == 1 || num[i] == 2 {
                    for j in 0..n {
                        flag[i][j] = 0;
                    }
                    cont_flag = true;
                }
            }
            if !cont_flag {
                break;
            }
        }

        // Calculate overlaps
        for i in 0..n {
            if num[i] > 0 {
                for j in 0..n {
                    if flag[i][j] == 1 {
                        for k in (j + 1)..n {
                            if flag[i][k] == 1 && flag[j][k] == 1 {
                                overlap[i][j] += 1;
                                overlap[i][k] += 1;
                            }
                        }
                    }
                }
            }
        }

        // Determine max and min overlaps
        let mut max = [0; MAX_N];
        let mut min = [1000; MAX_N];

        for i in 0..n {
            for j in 0..n {
                if max[i] < overlap[i][j] {
                    max[i] = overlap[i][j];
                }
                if overlap[i][j] > 0 && min[i] > overlap[i][j] {
                    min[i] = overlap[i][j];
                }
            }
        }

        for i in 0..n {
            for j in 0..n {
                if overlap[i][j] < overlap[i][i] || max[i] == min[i] {
                    if max_overlap < overlap[i][j] {
                        max_overlap = overlap[i][j];
                    }
                }
            }
        }

        println!("{}", max_overlap + 1);
    }
}
