use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Terminate if both n and m are zero
        if n == 0 && m == 0 {
            break;
        }

        // Initialize the connection matrix and the list of connected students
        let mut student = vec![vec![false; 501]; 501];
        let mut list = vec![false; 501];

        // Read connections
        for _ in 0..m {
            let line = reader.lines().next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let p1: usize = iter.next().unwrap().parse().unwrap();
            let p2: usize = iter.next().unwrap().parse().unwrap();
            student[p1][p2] = true;
            student[p2][p1] = true;
        }

        // Mark direct connections from student 1
        for i in 2..=n {
            if student[1][i] {
                list[i] = true;
            }
        }

        // Mark indirect connections
        for i in 2..=n {
            if list[i] {
                for j in 2..=n {
                    if student[i][j] {
                        list[j] = true;
                    }
                }
            }
        }

        // Count unique connected students excluding the first student
        let count = list.iter().skip(2).filter(|&&connected| connected).count();
        println!("{}", count);
    }
}
