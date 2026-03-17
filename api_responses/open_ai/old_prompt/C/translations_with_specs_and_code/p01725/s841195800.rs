// AOJ 2613: Unordered Operators
// Translated from C to Rust

use std::io::{self, Read};

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

fn get_int(p: &mut std::str::Chars) -> i64 {
    let mut n = 0;
    while let Some(c) = p.next() {
        if c.is_digit(10) {
            n = 10 * n + (c as i64 - '0' as i64);
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
        '*' => MUL,
        _ => unreachable!(),
    }
}

fn rank(op: i32, rid: usize, tbl: &[[i32; 3]; 13]) -> i32 {
    if op == LEFT {
        LEFT
    } else {
        tbl[rid][op as usize]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let str_input = input.trim();

    let tbl: [[i32; 3]; 13] = [
        [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1],
        [3, 1, 2], [3, 2, 1], [1, 1, 1], [1, 1, 2],
        [2, 2, 1], [1, 2, 1], [2, 1, 2], [1, 2, 2],
        [2, 1, 1],
    ];

    let mut ans = 0;
    let mut first = true;

    for rid in 0..13 {
        let mut stack: Vec<T> = Vec::new();
        let mut buf: Vec<T> = Vec::new();
        let mut p = str_input.chars();
        
        while let Some(c) = p.next() {
            if c.is_digit(10) {
                let value = get_int(&mut p);
                buf.push(T { id: VAL, v: value });
            } else if c == ')' {
                while let Some(top) = stack.last() {
                    if top.id == LEFT {
                        stack.pop();
                        break;
                    }
                    buf.push(stack.pop().unwrap());
                }
            } else if c == '(' {
                stack.push(T { id: LEFT, v: 0 });
            } else {
                let id = token(c);
                while let Some(top) = stack.last() {
                    if rank(id, rid, &tbl) <= rank(top.id, rid, &tbl) {
                        buf.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(T { id, v: 0 });
            }
        }
        
        while let Some(top) = stack.pop() {
            buf.push(top);
        }

        let mut top = 0;
        for i in 0..buf.len() {
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
                top -= 1;
            }
        }

        if first {
            ans = stack.pop().unwrap().v;
            first = false;
        } else {
            ans = ans.max(stack.pop().unwrap().v);
        }
    }

    println!("{}", ans);
}
