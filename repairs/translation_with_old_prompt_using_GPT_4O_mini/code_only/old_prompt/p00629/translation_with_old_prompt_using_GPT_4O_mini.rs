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
                let j_usize = j as usize;
                let j_minus_1_usize = (j - 1) as usize;
                if TT[j_usize].A > TT[j_minus_1_usize].A
                    || (TT[j_usize].A == TT[j_minus_1_usize].A && TT[j_usize].P < TT[j_minus_1_usize].P)
                    || (TT[j_usize].A == TT[j_minus_1_usize].A && TT[j_usize].P == TT[j_minus_1_usize].P && TT[j_usize].I < TT[j_minus_1_usize].I)
                {
                    TT.swap(j_usize, j_minus_1_usize);
                }
            }
        }

        let mut head = 0;
        for i in 0..n {
            let u_index = TT[i as usize].U as usize;
            UU[u_index] += 1;
            if head < 10 && UU[u_index] <= 3 {
                SS[head] = TT[i as usize].I;
                head += 1;
            } else if head < 20 && UU[u_index] <= 2 {
                SS[head] = TT[i as usize].I;
                head += 1;
            } else if head < 26 && UU[u_index] <= 1 {
                SS[head] = TT[i as usize].I;
                head += 1;
            }
        }

        for i in 0..head {
            println!("{}", SS[i]);
        }
    }
}
