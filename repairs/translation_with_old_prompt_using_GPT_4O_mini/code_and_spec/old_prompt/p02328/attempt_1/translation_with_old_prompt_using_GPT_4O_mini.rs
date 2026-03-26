use std::io::{self, BufRead};

const MAX: usize = 100001;

#[derive(Debug)]
struct Rect {
    height: i64,
    pos: usize,
}

fn large_histogram(w: usize, table: &mut [i64; MAX]) -> i64 {
    let mut sp = 0; // stack pointer
    let mut max_area = 0;
    table[w] = 0; // Set the sentinel value

    for i in 0..=w {
        let r = Rect {
            height: table[i],
            pos: i,
        };

        if sp == 0 || table[stack[sp - 1].pos] < r.height {
            stack[sp] = r;
            sp += 1;
        } else if table[stack[sp - 1].pos] > r.height {
            let mut target = i;

            while sp > 0 && table[stack[sp - 1].pos] >= r.height {
                let pre = stack[sp - 1];
                sp -= 1; // Pop from stack
                let area = pre.height * (i as i64 - pre.pos as i64);
                if max_area < area {
                    max_area = area;
                }
                target = pre.pos;
            }

            r.pos = target;
            stack[sp] = r;
            sp += 1;
        }
    }

    max_area
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let w: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut table: [i64; MAX] = [0; MAX];

    for i in 0..w {
        table[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    let max_area = large_histogram(w, &mut table);
    println!("{}", max_area);
}
