use std::io::{self, BufRead};

fn drop_ink();
fn taint_1();
fn taint_2();
fn taint_3();
fn untaint() -> i32;
fn mosttaint() -> i32;

static mut MAP: [[i32; 10]; 10] = [[0; 10]; 10]; // Using unsafe for mutable static array
static mut IN: [i32; 3] = [0; 3];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Err(_) = parse_input(&line) {
            break; // Stop on EOF or parse error
        }
        unsafe {
            drop_ink();
        }
    }
    println!("{}\n{}", untaint(), mosttaint());
}

fn parse_input(line: &str) -> Result<(), ()> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return Err(()); // Invalid input format
    }
    unsafe {
        IN[0] = parts[0].parse().map_err(|_| ())?;
        IN[1] = parts[1].parse().map_err(|_| ())?;
        IN[2] = parts[2].parse().map_err(|_| ())?;
    }
    Ok(())
}

fn drop_ink() {
    unsafe {
        match IN[2] {
            3 => taint_3(),
            2 => taint_2(),
            1 => taint_1(),
            _ => {}
        }
    }
}

fn taint_1() {
    unsafe {
        if IN[0] - 1 >= 0 {
            MAP[(IN[0] - 1) as usize][IN[1] as usize] += 1;
        }
        if IN[0] + 1 < 10 {
            MAP[(IN[0] + 1) as usize][IN[1] as usize] += 1;
        }
        if IN[1] - 1 >= 0 {
            MAP[IN[0] as usize][(IN[1] - 1) as usize] += 1;
        }
        if IN[1] + 1 < 10 {
            MAP[IN[0] as usize][(IN[1] + 1) as usize] += 1;
        }
    }
}

fn taint_2() {
    unsafe {
        if IN[0] - 1 >= 0 {
            if IN[1] - 1 >= 0 {
                MAP[(IN[0] - 1) as usize][(IN[1] - 1) as usize] += 1;
            }
            if IN[1] + 1 < 10 {
                MAP[(IN[0] - 1) as usize][(IN[1] + 1) as usize] += 1;
            }
        }
        if IN[0] + 1 < 10 {
            if IN[1] - 1 >= 0 {
                MAP[(IN[0] + 1) as usize][(IN[1] - 1) as usize] += 1;
            }
            if IN[1] + 1 < 10 {
                MAP[(IN[0] + 1) as usize][(IN[1] + 1) as usize] += 1;
            }
        }
    }
}

fn taint_3() {
    unsafe {
        if IN[0] - 2 >= 0 {
            MAP[(IN[0] - 2) as usize][IN[1] as usize] += 1;
        }
        if IN[0] + 2 < 10 {
            MAP[(IN[0] + 2) as usize][IN[1] as usize] += 1;
        }
        if IN[1] - 2 >= 0 {
            MAP[IN[0] as usize][(IN[1] - 2) as usize] += 1;
        }
        if IN[1] + 2 < 10 {
            MAP[IN[0] as usize][(IN[1] + 2) as usize] += 1;
        }
    }
}

fn untaint() -> i32 {
    let mut ret = 0;
    unsafe {
        for p in 0..10 {
            for q in 0..10 {
                if MAP[p][q] == 0 {
                    ret += 1;
                }
            }
        }
    }
    ret
}

fn mosttaint() -> i32 {
    let mut ret = 0;
    unsafe {
        for p in 0..10 {
            for q in 0..10 {
                if MAP[p][q] > ret {
                    ret = MAP[p][q];
                }
            }
        }
    }
    ret
}
