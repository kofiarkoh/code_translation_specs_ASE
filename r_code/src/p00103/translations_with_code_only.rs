use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        let mut score = 0;
        let mut base = [0; 3];
        let mut out = 0;

        while out < 3 {
            let event = lines.next().unwrap().unwrap();

            if event == "HIT" {
                base[0] += 1;
                for i in 0..3 {
                    if base[i] > 1 {
                        if i < 2 {
                            base[i + 1] += 1;
                        } else {
                            score += 1;
                        }
                        base[i] -= 1;
                    }
                }
            } else if event == "HOMERUN" {
                for i in 0..3 {
                    if base[i] == 1 {
                        score += 1;
                        base[i] = 0;
                    }
                }
                score += 1;
            } else if event == "OUT" {
                out += 1;
            }
        }
        println!("{}", score);
    }
}
