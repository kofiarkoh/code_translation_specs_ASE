use std::io::{self, BufRead};

const MAX: usize = 100;
type Farm = [[char; MAX]; MAX];

fn divide(farm: &mut Farm, i: usize, j: usize, h: usize, w: usize, moji: char) {
    if farm[i][j] == moji {
        farm[i][j] = '+';
        if j + 1 < w {
            divide(farm, i, j + 1, h, w, moji);
        }
        if i + 1 < h {
            divide(farm, i + 1, j, h, w, moji);
        }
        if j > 0 {
            divide(farm, i, j - 1, h, w, moji);
        }
        if i > 0 {
            divide(farm, i - 1, j, h, w, moji);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut farm: Farm = [['\0'; MAX]; MAX];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let h: usize = iter.next().unwrap().parse().unwrap();
        let w: usize = iter.next().unwrap().parse().unwrap();

        if h == 0 && w == 0 {
            break;
        }

        for i in 0..h {
            let input_line = reader.lines().next().unwrap().unwrap();
            for (j, c) in input_line.chars().enumerate() {
                farm[i][j] = c;
            }
        }

        let mut cnt = 0;
        for i in 0..h {
            for j in 0..w {
                if farm[i][j] == '+' {
                    continue;
                }
                cnt += 1;
                divide(&mut farm, i, j, h, w, farm[i][j]);
            }
        }
        println!("{}", cnt);
    }
}
