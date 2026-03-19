use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct T {
    I: i32,
    U: i32,
    A: i32,
    P: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut TT: Vec<T> = Vec::with_capacity(300);
    let mut UU = vec![0; 1001];
    let mut SS = vec![0; 1001];

    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        TT.clear();
        for _ in 0..n {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap();
            let values: Vec<i32> = input.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            TT.push(T {
                I: values[0],
                U: values[1],
                A: values[2],
                P: values[3],
            });
        }

        // Bubble sort based on the specified criteria
        for i in 0..n {
            for j in (1..n - i as usize).rev() {
                if TT[j].A > TT[j - 1].A
                    || (TT[j].A == TT[j - 1].A && TT[j].P < TT[j - 1].P)
                    || (TT[j].A == TT[j - 1].A && TT[j].P == TT[j - 1].P && TT[j].I < TT[j - 1].I)
                {
                    TT.swap(j, j - 1);
                }
            }
        }

        let mut head = 0;
        for i in 0..n {
            UU[TT[i].U as usize] += 1;
            if head < 10 && UU[TT[i].U as usize] <= 3 {
                SS[head] = TT[i].I;
                head += 1;
            } else if head < 20 && UU[TT[i].U as usize] <= 2 {
                SS[head] = TT[i].I;
                head += 1;
            } else if head < 26 && UU[TT[i].U as usize] <= 1 {
                SS[head] = TT[i].I;
                head += 1;
            }
        }

        for i in 0..head {
            println!("{}", SS[i]);
        }
    }
}
