use std::io::{self, BufRead};

const MAX: usize = 210;

const VAL: i32 = 10;
const LEFT: i32 = -1;
const RIGHT: i32 = -2;
const PLUS: i32 = 0;
const MINUS: i32 = 1;
const MUL: i32 = 2;

#[derive(Clone, Copy)]
struct T {
    v: i64,
    id: i32,
}

fn get_int(p: &mut &str) -> i64 {
    let mut n = 0;
    while let Some(c) = p.chars().next() {
        if c.is_digit(10) {
            n = 10 * n + (c as i64 - '0' as i64);
            *p = &p[1..]; // Move the pointer forward
        } else {
            break;
        }
    }
    n
}

fn token(c: char) -> i32 {
    match c {
        '+' => PLUS,
        '-' => MINUS,
        _ => MUL,
    }
}

fn rank(op: i32, rid: usize) -> i32 {
    if op == LEFT {
        LEFT
    } else {
        TABLE[rid][op as usize]
    }
}

const TABLE: [[i32; 3]; 13] = [
    [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1],
    [3, 1, 2], [3, 2, 1], [1, 1, 1], [1, 1, 2],
    [2, 2, 1], [1, 2, 1], [2, 1, 2], [1, 2, 2],
    [2, 1, 1],
];

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut f = true;
    let mut ans = 0;

    for rid in 0..13 {
        let mut stack: Vec<T> = Vec::new();
        let mut buf: Vec<T> = Vec::new();
        let mut top = 0;
        let mut end = 0;
        let mut p = &input;

        while !p.is_empty() {
            let c = p.chars().next().unwrap();
            if c.is_digit(10) {
                buf.push(T { id: VAL, v: get_int(&mut p) });
                end += 1;
            } else if c == ')' {
                while top > 0 && stack[top - 1].id != LEFT {
                    buf.push(stack.pop().unwrap());
                    end += 1;
                }
                p = &p[1..]; // Move past ')'
                top -= 1;
            } else if c == '(' {
                stack.push(T { id: LEFT, v: 0 });
                top += 1;
                p = &p[1..]; // Move past '('
            } else {
                let id = token(c);
                p = &p[1..]; // Move past operator
                while top > 0 {
                    if rank(id, rid) <= rank(stack[top - 1].id, rid) {
                        buf.push(stack.pop().unwrap());
                        end += 1;
                    } else {
                        break;
                    }
                }
                stack.push(T { id, v: 0 });
                top += 1;
            }
        }
        while top > 0 {
            buf.push(stack.pop().unwrap());
            end += 1;
        }

        top = 0;
        for i in 0..end {
            let id = buf[i].id;
            if id == VAL {
                stack.push(T { v: buf[i].v, id });
                top += 1;
            } else {
                let v1 = stack.pop().unwrap().v;
                let v2 = stack.pop().unwrap().v;
                let result = match id {
                    PLUS => v2 + v1,
                    MINUS => v2 - v1,
                    MUL => v2 * v1,
                    _ => unreachable!(),
                };
                stack.push(T { v: result, id });
                top += 1;
            }
        }
        if f {
            ans = stack.pop().unwrap().v;
            f = false;
        } else if ans < stack.pop().unwrap().v {
            ans = stack[top - 1].v;
        }
    }
    println!("{}", ans);
}
