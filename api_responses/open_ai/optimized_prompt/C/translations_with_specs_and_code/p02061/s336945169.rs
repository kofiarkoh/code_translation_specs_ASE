use std::io::{self, BufRead};

const MAX_SIZE: usize = 114514;

fn minaseinori(idx: usize, acc: i64, fukuennmisato: &[(i64, i64)], utiyamayumi: i64, yosinagasayuri: &mut i64) {
    if idx >= fukuennmisato.len() {
        if acc != utiyamayumi {
            *yosinagasayuri *= acc;
            if *yosinagasayuri > utiyamayumi * 2 {
                *yosinagasayuri = utiyamayumi * 2;
            }
        }
    } else {
        let (base, exp) = fukuennmisato[idx];
        for _ in 0..=*exp {
            minaseinori(idx + 1, acc, fukuennmisato, utiyamayumi, yosinagasayuri);
            acc *= base;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let Q: usize = lines.next().unwrap().unwrap().parse().expect("Failed to read Q");
    let mut N = Vec::with_capacity(Q);
    let mut max = 0;

    for _ in 0..Q {
        let n: i64 = lines.next().unwrap().unwrap().parse().expect("Failed to read N");
        N.push(n);
        if n > max {
            max = n;
        }
    }

    let mut count = vec![0; (max + 1) as usize];

    for i in 2..=max {
        let mut sumipe = i;
        let mut fukuennmisato = Vec::new();

        for tomatu in 2..=((sumipe as f64).sqrt() as i64) {
            if sumipe % tomatu == 0 {
                let mut c = 0;
                while sumipe % tomatu == 0 {
                    sumipe /= tomatu;
                    c += 1;
                }
                fukuennmisato.push((tomatu, c));
            }
        }
        if sumipe > 1 {
            fukuennmisato.push((sumipe, 1));
        }

        let utiyamayumi = i;
        let mut yosinagasayuri = 1;
        minaseinori(0, 1, &fukuennmisato, utiyamayumi, &mut yosinagasayuri);
        count[i as usize] = count[(i - 1) as usize] + (yosinagasayuri >= i * 2);
    }

    for &n in &N {
        println!("{}", count[n as usize]);
    }
}
