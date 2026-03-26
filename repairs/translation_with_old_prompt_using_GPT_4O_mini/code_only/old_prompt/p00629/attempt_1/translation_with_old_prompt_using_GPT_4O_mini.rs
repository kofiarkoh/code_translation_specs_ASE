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
    let mut lines = reader.lines();

    let mut TT: Vec<T> = Vec::new();
    let mut UU = vec![0; 1001];
    let mut SS = vec![0; 26];

    loop {
        let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        TT.clear();
        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let I: i32 = parts.next().unwrap().parse().unwrap();
            let U: i32 = parts.next().unwrap().parse().unwrap();
            let A: i32 = parts.next().unwrap().parse().unwrap();
            let P: i32 = parts.next().unwrap().parse().unwrap();
            TT.push(T { I, U, A, P });
        }

        // Bubble sort based on A, P, and I
        for i in 0..n {
            for j in (1..n - i).rev() {
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
