// AOJ 2613: Unordered Operators
// Translated from C to Rust

use std::io::{self, Read};
use std::str::FromStr;

const MAX: usize = 210;

const VAL: i32 = 10;
const LEFT: i32 = -1;
const RIGHT: i32 = -2;
const PLUS: i32 = 0;
const MINUS: i32 = 1;
const MUL: i32 = 2;

#[derive(Clone, Copy)]
struct Token {
    v: i64,
    id: i32,
}

fn get_int(p: &mut &str) -> i64 {
    let mut n = 0;
    while let Some(c) = p.chars().next() {
        if c.is_digit(10) {
            n = 10 * n + (c.to_digit(10).unwrap() as i64);
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
        '*' => MUL,
        _ => panic!("Unexpected character"),
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let expression = input.trim();

    let mut ans = i64::MAX;
    for rid in 0..13 {
        let mut stack: Vec<Token> = Vec::new();
        let mut buf: Vec<Token> = Vec::new();
        let mut p = expression;

        while !p.is_empty() {
            if p.chars().next().unwrap().is_digit(10) {
                buf.push(Token { id: VAL, v: get_int(&mut p) });
            } else if p.starts_with(')') {
                while let Some(top_token) = stack.last() {
                    if top_token.id == LEFT {
                        break;
                    }
                    buf.push(stack.pop().unwrap());
                }
                p = &p[1..]; // Move past ')'
                stack.pop(); // Remove the '('
            } else if p.starts_with('(') {
                stack.push(Token { id: LEFT, v: 0 });
                p = &p[1..]; // Move past '('
            } else {
                let id = token(p.chars().next().unwrap());
                p = &p[1..]; // Move past operator
                while let Some(top_token) = stack.last() {
                    if rank(id, rid) <= rank(top_token.id, rid) {
                        buf.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(Token { id, v: 0 });
            }
        }
        while let Some(top_token) = stack.pop() {
            buf.push(top_token);
        }

        let mut top = 0;
        for i in 0..buf.len() {
            let id = buf[i].id;
            if id == VAL {
                stack.push(Token { v: buf[i].v, id: 0 });
                top += 1;
            } else {
                let v1 = stack.pop().unwrap().v;
                let v2 = stack.pop().unwrap().v;
                let result = match id {
                    PLUS => v2 + v1,
                    MINUS => v2 - v1,
                    MUL => v2 * v1,
                    _ => panic!("Unexpected operator"),
                };
                stack.push(Token { v: result, id: 0 });
                top -= 1;
            }
        }
        if top == 1 {
            let current_result = stack.pop().unwrap().v;
            ans = ans.min(current_result);
        }
    }
    println!("{}", ans);
}
