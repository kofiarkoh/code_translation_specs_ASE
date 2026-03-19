use std::io::{self, BufRead};

const MAX: usize = 100001;

#[derive(Clone, Copy)]
struct Rect {
    height: i64,
    pos: usize,
}

fn large_histogram(w: usize, table: &[i64; MAX]) -> i64 {
    let mut sp = 0;
    let mut max = 0;
    let mut stack: [Rect; MAX] = [Rect { height: 0, pos: 0 }; MAX]; // Initialize stack with default values
    let mut table_with_zero = [0; MAX];
    table_with_zero[..w].copy_from_slice(&table[..w]);
    table_with_zero[w] = 0; // Set the sentinel value

    for i in 0..=w {
        let r = Rect {
            height: table_with_zero[i],
            pos: i,
        };

        if sp == 0 || stack[sp - 1].height < r.height {
            stack[sp] = r;
            sp += 1;
        } else if stack[sp - 1].height > r.height {
            let mut target = i;
            while sp > 0 && stack[sp - 1].height >= r.height {
                let pre = stack[sp - 1];
                sp -= 1; // Pop the stack
                let area = pre.height * (i as i64 - pre.pos as i64);
                if max < area {
                    max = area;
                }
                target = pre.pos;
            }
            let mut new_r = r;
            new_r.pos = target;
            stack[sp] = new_r;
            sp += 1;
        }
    }
    max
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let w: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Changed to usize
    let mut table: [i64; MAX] = [0; MAX];

    for i in 0..w {
        table[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    let max_area = large_histogram(w, &table);
    println!("{}", max_area);
}
