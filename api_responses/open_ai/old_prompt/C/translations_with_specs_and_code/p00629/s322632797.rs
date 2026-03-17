use std::io::{self, BufRead};

#[derive(Debug)]
struct Item {
    I: i32,
    U: i32,
    A: i32,
    P: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    loop {
        let mut n_str = String::new();
        input.read_line(&mut n_str).unwrap();
        let n: usize = n_str.trim().parse().unwrap();
        
        if n == 0 {
            break;
        }

        let mut items: Vec<Item> = Vec::with_capacity(n);
        let mut uu = vec![0; 1001];
        let mut ss = vec![0; 26];

        for _ in 0..n {
            let mut line = String::new();
            input.read_line(&mut line).unwrap();
            let parts: Vec<i32> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            items.push(Item {
                I: parts[0],
                U: parts[1],
                A: parts[2],
                P: parts[3],
            });
        }

        // Sort items based on A, P, and I
        items.sort_by(|a, b| {
            let a_cmp = a.A.cmp(&b.A)
                .then_with(|| a.P.cmp(&b.P))
                .then_with(|| a.I.cmp(&b.I));
            a_cmp.reverse() // Sort in descending order for A, and ascending for P and I
        });

        let mut head = 0;

        for item in items.iter() {
            uu[item.U as usize] += 1;

            if head < 10 && uu[item.U as usize] <= 3 {
                ss[head] = item.I;
                head += 1;
            } else if head < 20 && uu[item.U as usize] <= 2 {
                ss[head] = item.I;
                head += 1;
            } else if head < 26 && uu[item.U as usize] <= 1 {
                ss[head] = item.I;
                head += 1;
            }
        }

        for i in 0..head {
            println!("{}", ss[i]);
        }
    }
}
