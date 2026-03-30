use std::io::{self, BufRead};

const MAX_K: usize = 5000;
const MAX_N: usize = 1000;

fn get_len(an: usize, bn: usize, ais: &[i32], bis: &[i32], k0: usize) -> usize {
    let mut ai = 0;
    for bi in k0..bn {
        if ai >= an {
            return bi - k0;
        }
        let b = bis[bi];
        let mut aj = ai;
        while aj < an && ais[aj] != b {
            aj += 1;
        }
        if aj >= an {
            return bi - k0;
        }
        ai = aj + 1;
    }
    bn - k0
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let an: usize = lines.next().unwrap().unwrap().parse().unwrap_or(0);
    let bn: usize = lines.next().unwrap().unwrap().parse().unwrap_or(0);

    let mut ais = Vec::with_capacity(an);
    let mut bis = Vec::with_capacity(bn);

    for _ in 0..an {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(value) = line.parse::<i32>() {
                ais.push(value);
            }
        }
    }
    for _ in 0..bn {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(value) = line.parse::<i32>() {
                bis.push(value);
            }
        }
    }

    let mut max_len = 0;

    for k0 in 0..bn {
        if k0 + max_len > bn {
            break;
        }
        let len = get_len(an, bn, &ais, &bis, k0);
        if max_len < len {
            max_len = len;
        }
    }

    println!("{}", max_len);
}
