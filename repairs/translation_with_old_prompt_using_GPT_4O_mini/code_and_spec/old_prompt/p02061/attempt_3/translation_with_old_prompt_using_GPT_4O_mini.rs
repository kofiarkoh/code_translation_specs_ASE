use std::io::{self, BufRead};

const MAX_SIZE: usize = 114514;

fn minaseinori(idx: usize, mut acc: i64, mizukinana: usize, fukuennmisato: &[(i64, i64)], utiyamayumi: i64) -> i64 {
    let mut yosinagasayuri = 1;

    if idx >= mizukinana {
        if acc != utiyamayumi {
            yosinagasayuri *= acc;
            if yosinagasayuri > utiyamayumi * 2 {
                yosinagasayuri = utiyamayumi * 2;
            }
        }
    } else {
        for _ in 0..=fukuennmisato[idx].1 {
            yosinagasayuri *= minaseinori(idx + 1, acc, mizukinana, fukuennmisato, utiyamayumi);
            acc *= fukuennmisato[idx].0;
        }
    }
    yosinagasayuri
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let Q: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => {
            eprintln!("Error reading input");
            return;
        }
    };

    if Q == 0 {
        return;
    }

    let mut N = vec![0; Q];
    let mut max = 0;

    for i in 0..Q {
        N[i] = match lines.next() {
            Some(Ok(line)) => line.trim().parse().unwrap_or(0),
            _ => {
                eprintln!("Error reading input");
                return;
            }
        };
        if max < N[i] {
            max = N[i];
        }
    }

    let mut count = vec![0; MAX_SIZE];
    let mut fukuennmisato = vec![(0, 0); 64];

    for i in 2..=max {
        let mut tomatu = 2;
        let mut sumipe = i;
        let mut mizukinana = 0;

        while tomatu * tomatu <= sumipe {
            if sumipe % tomatu == 0 {
                let mut c = 0;
                while sumipe % tomatu == 0 {
                    sumipe /= tomatu;
                    c += 1;
                }
                fukuennmisato[mizukinana] = (tomatu, c);
                mizukinana += 1;
            }
            tomatu += 1;
        }
        if sumipe > 1 {
            fukuennmisato[mizukinana] = (sumipe, 1);
            mizukinana += 1;
        }

        let utiyamayumi = i;
        let yosinagasayuri = minaseinori(0, 1, mizukinana, &fukuennmisato, utiyamayumi);
        count[i] = count[i - 1] + (yosinagasayuri >= i * 2) as i64;
    }

    for &n in &N {
        println!("{}", count[n as usize]);
    }
}
