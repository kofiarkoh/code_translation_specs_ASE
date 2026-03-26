use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let mut nums = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let m = nums.next().unwrap();
        
        if n == 0 && m == 0 {
            break;
        }

        let mut q = vec![vec![0; m as usize]; n as usize];
        for j in 0..n {
            let mut row_input = String::new();
            reader.read_line(&mut row_input).unwrap();
            let row_nums = row_input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            for (i, value) in row_nums.enumerate() {
                q[j as usize][i] = value;
            }
        }

        let mut cnt = vec![0; m as usize];
        for i in 0..n {
            for j in 0..m {
                if i == 0 {
                    cnt[j as usize] = 0;
                }
                cnt[j as usize] += q[i as usize][j as usize];
            }
        }

        let mut maxed = vec![0; m as usize];
        for _ in 0..m {
            let mut max = -1;
            for j in 0..m {
                if maxed[j as usize] == 0 {
                    if max == -1 {
                        max = j;
                    } else if cnt[max as usize] < cnt[j as usize] {
                        max = j;
                    }
                }
            }
            if max != -1 {
                if _ > 0 {
                    print!(" ");
                }
                print!("{}", max + 1);
                maxed[max as usize] = 1;
            }
        }
        println!();
    }
}
