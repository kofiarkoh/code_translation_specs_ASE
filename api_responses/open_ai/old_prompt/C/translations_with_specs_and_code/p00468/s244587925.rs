use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    loop {
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let m = nums.next().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        let mut student = [[0; 501]; 501];
        let mut list = [0; 501];

        for _ in 0..m {
            let mut line = String::new();
            input.read_line(&mut line).unwrap();
            let mut edges = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            let p1 = edges.next().unwrap();
            let p2 = edges.next().unwrap();
            student[p1][p2] = 1;
            student[p2][p1] = 1;
        }

        for i in 2..=n {
            if student[1][i] == 1 {
                list[i] = 1;
            }
        }

        for i in 2..=n {
            if student[1][i] == 1 {
                for j in 2..=n {
                    if student[i][j] == 1 {
                        list[j] = 1;
                    }
                }
            }
        }

        let count: usize = list.iter().skip(2).sum();
        println!("{}", count);
    }
}
