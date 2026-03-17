use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Maximum number of students
    const MAX_STUDENTS: usize = 501;

    // Initialize the adjacency matrix and the list of connected students
    let mut student = [[0; MAX_STUDENTS]; MAX_STUDENTS];
    let mut list = [0; MAX_STUDENTS];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        
        // Read n and m
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Termination condition
        if n == 0 && m == 0 {
            break;
        }

        // Reset the connection matrix and the list for each dataset
        for i in 0..MAX_STUDENTS {
            list[i] = 0;
            for j in 0..MAX_STUDENTS {
                student[i][j] = 0;
            }
        }

        // Read connections
        for _ in 0..m {
            let line = reader.lines().next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let p1: usize = iter.next().unwrap().parse().unwrap();
            let p2: usize = iter.next().unwrap().parse().unwrap();
            student[p1][p2] = 1;
            student[p2][p1] = 1;
        }

        // Mark direct connections from student 1
        for i in 2..=n {
            if student[1][i] == 1 {
                list[i] = 1;
            }
        }

        // Mark indirect connections
        for i in 2..=n {
            if student[1][i] == 1 {
                for j in 2..=n {
                    if student[i][j] == 1 {
                        list[j] = 1;
                    }
                }
            }
        }

        // Count connected students
        let count: usize = list.iter().skip(2).take(n - 1).sum();
        println!("{}", count);
    }
}
