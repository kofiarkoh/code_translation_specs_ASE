use std::io::{self, BufRead};

fn search(s: &[i32], key: i32, head: usize, tail: usize) -> isize {
    if head > tail {
        return -1;
    }

    let mid = (head + tail) / 2;

    if s[mid] == key {
        return mid as isize;
    } else if s[mid] > key {
        return search(s, key, head, mid - 1);
    } else {
        return search(s, key, mid + 1, tail);
    }
}

fn get_lb(s: &[i32], key: i32, head: usize, tail: usize) -> usize {
    if s[0] > key || tail == usize::MAX {
        return 0;
    }
    if s[tail] < key || head > tail {
        return tail + 1;
    }

    let mid = (head + tail) / 2;

    if s[mid] < key {
        return get_lb(s, key, mid + 1, tail);
    } else if s[mid] >= key {
        if mid > 0 && s[mid - 1] < key {
            return mid;
        } else {
            return get_lb(s, key, head, mid - 1);
        }
    }

    tail + 1
}

fn insert(s: &mut Vec<i32>, key: i32, cnt: usize) -> i32 {
    let d = if cnt == 0 { 0 } else { get_lb(s, key, 0, cnt - 1) };

    s.insert(d, key);
    1
}

fn delete(s: &mut Vec<i32>, key: i32, cnt: usize) -> i32 {
    let d = search(s, key, 0, cnt - 1);
    if d < 0 {
        return -1;
    }

    s.remove(d as usize);
    1
}

fn dump(s: &[i32], l: i32, r: i32, cnt: usize) {
    let mut i = get_lb(s, l, 0, cnt - 1);

    while i < cnt && s[i] <= r {
        println!("{}", s[i]);
        i += 1;
    }
}

fn print(s: &[i32], cnt: usize) {
    print!("cnt={},s:", cnt);
    for i in 0..cnt {
        print!(" {}", s[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut s: Vec<i32> = Vec::with_capacity(q);
    let mut cnt = 0;

    for _ in 0..q {
        let com: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        match com {
            0 => {
                let x: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
                if search(&s, x, 0, cnt - 1) == -1 {
                    if insert(&mut s, x, cnt) == 1 {
                        cnt += 1;
                    }
                }
                println!("{}", cnt);
            }
            1 => {
                let x: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
                if search(&s, x, 0, cnt - 1) == -1 {
                    println!("0");
                } else {
                    println!("1");
                }
            }
            2 => {
                let x: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
                if delete(&mut s, x, cnt) == 1 {
                    cnt -= 1;
                }
            }
            3 => {
                let (x, y): (i32, i32) = {
                    let line = lines.next().unwrap().unwrap();
                    let mut parts = line.split_whitespace();
                    let x = parts.next().unwrap().parse().unwrap();
                    let y = parts.next().unwrap().parse().unwrap();
                    (x, y)
                };
                dump(&s, x, y, cnt);
            }
            _ => {}
        }
    }
}
