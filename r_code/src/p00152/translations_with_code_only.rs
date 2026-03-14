use std::io::{self, BufRead};

#[derive(Debug)]
struct Info {
    no: i32,
    score: i32,
}

fn sort_info(n: usize, t: &mut [Info]) {
    for i in 0..n - 1 {
        for j in i + 1..n {
            if t[i].score < t[j].score || (t[i].score == t[j].score && t[i].no > t[j].no) {
                t.swap(i, j);
            }
        }
    }
}

fn scoring() -> i32 {
    let stdin = io::stdin();
    let mut total = 0;
    let mut sum = 0;
    let mut frame = 1;
    let mut time = 1;
    let mut flg_spare = false;
    let mut flg_strike = 0;

    while frame < 10 {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let x: i32 = input.trim().parse().unwrap();
        sum += x;

        if flg_spare {
            total += x;
            flg_spare = false;
        } else if flg_strike > 0 {
            if flg_strike == 3 {
                total += x;
                flg_strike -= 1;
            }
            total += x;
            flg_strike -= 1;
        }

        if time == 2 {
            total += sum;
            if sum == 10 {
                flg_spare = true;
            }
            sum = 0;
            time = 1;
            frame += 1;
        } else if time == 1 {
            time += 1;
            if x == 10 {
                total += 10;
                flg_strike += 2;
                sum = 0;
                time = 1;
                frame += 1;
            }
        }
    }

    for i in 0..3 {
        if i == 2 && sum < 10 {
            break;
        }

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let x: i32 = input.trim().parse().unwrap();
        total += x;
        sum += x;

        if flg_spare {
            total += x;
            flg_spare = false;
        } else if flg_strike > 0 {
            if flg_strike == 3 {
                total += x;
                flg_strike -= 1;
            }
            total += x;
            flg_strike -= 1;
        }
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let mut info: Vec<Info> = Vec::new();

    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        for _ in 0..n {
            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();
            let no: i32 = input.trim().parse().unwrap();
            let score = scoring();
            info.push(Info { no, score });
        }

        sort_info(info.len(), &mut info);
        for i in 0..info.len() {
            println!("{} {}", info[i].no, info[i].score);
        }
        info.clear(); // Clear the vector for the next iteration
    }
}
