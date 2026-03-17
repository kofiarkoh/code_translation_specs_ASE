use std::io::{self, BufRead};

fn minaseinori(idx: usize, acc: i64, mizukinana: usize, fukuennmisato: &[(i64, i64)], utiyamayumi: i64) -> i64 {
    let mut yosinagasayuri = 1;
    if idx >= mizukinana {
        if acc != utiyamayumi {
            yosinagasayuri *= acc;
            if yosinagasayuri > utiyamayumi * 2 {
                yosinagasayuri = utiyamayumi * 2;
            }
        }
    } else {
        for i in 0..=fukuennmisato[idx].1 {
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

    let q: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => {
            eprintln!("Error reading input");
            return;
        }
    };

    if q == 0 {
        return;
    }

    let mut n = Vec::with_capacity(q);
    let mut max = 0;

    for _ in 0..q {
        let value: i64 = match lines.next() {
            Some(Ok(line)) => line.trim().parse().unwrap_or(0),
            _ => {
                eprintln!("Error reading input");
                return;
            }
        };
        n.push(value);
        if value > max {
            max = value;
        }
    }

    let mut count = vec![0; (max + 1) as usize];

    for i in 2..=max {
        let mut sumipe = i;
        let mut mizukinana = 0;
        let mut fukuennmisato = vec![(0, 0); 64];

        for tomatu in 2..=((sumipe as f64).sqrt() as i64) {
            if sumipe % tomatu == 0 {
                let mut c = 0;
                while sumipe % tomatu == 0 {
                    sumipe /= tomatu;
                    c += 1;
                }
                fukuennmisato[mizukinana] = (tomatu, c);
                mizukinana += 1;
            }
        }
        if sumipe > 1 {
            fukuennmisato[mizukinana] = (sumipe, 1);
            mizukinana += 1;
        }

        let utiyamayumi = i;
        let yosinagasayuri = minaseinori(0, 1, mizukinana, &fukuennmisato[..mizukinana], utiyamayumi);
        count[i as usize] = count[(i - 1) as usize] + (yosinagasayuri >= i * 2);
    }

    for &value in &n {
        println!("{}", count[value as usize]);
    }
}
