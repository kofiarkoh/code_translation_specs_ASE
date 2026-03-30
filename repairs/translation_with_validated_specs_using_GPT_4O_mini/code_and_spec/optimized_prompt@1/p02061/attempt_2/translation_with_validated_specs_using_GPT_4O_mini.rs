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
        let mut current_acc = acc;
        for _ in 0..=*exp {
            minaseinori(idx + 1, current_acc, fukuennmisato, utiyamayumi, yosinagasayuri);
            current_acc *= base;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let q: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for Q");
    let mut n = Vec::with_capacity(q);
    let mut max = 0;

    for _ in 0..q {
        let value: i32 = lines.next().unwrap().unwrap().parse().expect("Invalid input for N");
        n.push(value);
        if value > max {
            max = value;
        }
    }

    let mut count = vec![0; (max + 1) as usize];

    for i in 2..=max {
        let mut fukuennmisato = Vec::new();
        let mut sumipe = i;

        for tomatu in 2..=((sumipe as f64).sqrt() as i32) {
            if sumipe % tomatu == 0 {
                let mut c = 0;
                while sumipe % tomatu == 0 {
                    sumipe /= tomatu;
                    c += 1;
                }
                fukuennmisato.push((tomatu as i64, c as i64));
            }
        }
        if sumipe > 1 {
            fukuennmisato.push((sumipe as i64, 1));
        }

        let utiyamayumi = i as i64;
        let mut yosinagasayuri = 1;
        minaseinori(0, 1, &fukuennmisato, utiyamayumi, &mut yosinagasayuri);
        count[i as usize] = count[(i - 1) as usize] + if yosinagasayuri >= utiyamayumi * 2 { 1 } else { 0 };
    }

    for &value in n.iter() {
        println!("{}", count[value as usize]);
    }
}
