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
    let mut reader = stdin.lock(); // Changed to mutable
    let mut TT: Vec<T> = Vec::with_capacity(300);
    let mut UU = vec![0; 1001];
    let mut SS = vec![0; 26]; // Adjusted size to 26 to match the original logic

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
            for j in (1..(n - i)).rev() { // Changed to use i32 directly
                if TT[j as usize].A > TT[(j - 1) as usize].A
                    || (TT[j as usize].A == TT[(j - 1) as usize].A && TT[j as usize].P < TT[(j - 1) as usize].P)
                    || (TT[j as usize].A == TT[(j - 1) as usize].A && TT[j as usize].P == TT[(j - 1) as usize].P && TT[j as usize].I < TT[(j - 1) as usize].I)
                {
                    TT.swap(j as usize, (j - 1) as usize);
                }
            }
        }

        let mut head = 0;
        for i in 0..n {
            UU[TT[i as usize].U as usize] += 1;
            if head < 10 && UU[TT[i as usize].U as usize] <= 3 {
                SS[head] = TT[i as usize].I;
                head += 1;
            } else if head < 20 && UU[TT[i as usize].U as usize] <= 2 {
                SS[head] = TT[i as usize].I;
                head += 1;
            } else if head < 26 && UU[TT[i as usize].U as usize] <= 1 {
                SS[head] = TT[i as usize].I;
                head += 1;
            }
        }

        for i in 0..head {
            println!("{}", SS[i]);
        }
    }
}
