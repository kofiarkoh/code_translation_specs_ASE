use std::io::{self, BufRead};

const SIZE: usize = 9;

fn check_box(num: &[[i32; SIZE]; SIZE], flag: &mut [[i32; SIZE]; SIZE], y: usize, x: usize) {
    let mut cnt = [0; 10];

    for i in y..y + 3 {
        for j in x..x + 3 {
            cnt[num[i][j] as usize] += 1;
        }
    }
    for i in y..y + 3 {
        for j in x..x + 3 {
            if cnt[num[i][j] as usize] != 1 {
                flag[i][j] = 1;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..n {
        let mut num = [[0; SIZE]; SIZE];
        let mut flag = [[0; SIZE]; SIZE];

        for i in 0..SIZE {
            let mut cnt = [0; 10];
            let line = lines.next().unwrap().unwrap();
            let values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            for j in 0..SIZE {
                num[i][j] = values[j];
                cnt[num[i][j] as usize] += 1;
            }
            for j in 0..SIZE {
                if cnt[num[i][j] as usize] != 1 {
                    flag[i][j] = 1;
                }
            }
        }

        for j in 0..SIZE {
            let mut cnt = [0; 10];
            for i in 0..SIZE {
                cnt[num[i][j] as usize] += 1;
            }
            for i in 0..SIZE {
                if cnt[num[i][j] as usize] != 1 {
                    flag[i][j] = 1;
                }
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                check_box(&num, &mut flag, i * 3, j * 3);
            }
        }

        for i in 0..SIZE {
            for j in 0..SIZE {
                print!("{}{}", if flag[i][j] == 1 { '*' } else { ' ' }, num[i][j]);
            }
            println!();
        }
        println!();
    }
}
