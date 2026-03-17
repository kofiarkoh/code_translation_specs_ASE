// AOJ 2613: Unordered Operators
// 2017.10.10 bal4u@uu

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
struct T {
    v: i64,
    id: i32,
}

struct Stack {
    data: Vec<T>,
    top: usize,
}

impl Stack {
    fn new() -> Self {
        Stack {
            data: Vec::with_capacity(MAX + 3),
            top: 0,
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
        self.top += 1;
    }

    fn pop(&mut self) -> T {
        self.top -= 1;
        self.data.pop().unwrap() // Safe because we ensure stack is not empty
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let str_input = input.trim();

    let mut f = true;
    let mut ans = 0;

    for rid in 0..13 {
        let mut p = str_input;
        let mut stack = Stack::new();
        let mut buf = Stack::new();

        while !p.is_empty() {
            let c = p.chars().next().unwrap();
            p = &p[1..]; // Move the pointer forward

            if c.is_digit(10) {
                buf.push(T { id: VAL, v: get_int(&mut p) });
            } else if c == ')' {
                while !stack.is_empty() && stack.data[stack.top - 1].id != LEFT {
                    buf.push(stack.pop());
                }
                // Pop the left parenthesis
                if !stack.is_empty() {
                    stack.pop();
                }
            } else if c == '(' {
                stack.push(T { id: LEFT, v: 0 });
            } else {
                let id = token(c);
                while !stack.is_empty() {
                    if rank(id, rid) <= rank(stack.data[stack.top - 1].id, rid) {
                        buf.push(stack.pop());
                    } else {
                        break;
                    }
                }
                stack.push(T { id, v: 0 });
            }
        }

        while !stack.is_empty() {
            buf.push(stack.pop());
        }

        let mut top = 0;
        for i in 0..buf.top {
            let id = buf.data[i].id;
            if id == VAL {
                stack.push(T { v: buf.data[i].v, id });
            } else {
                let v1 = stack.pop().v;
                let v2 = stack.pop().v;
                let result = match id {
                    PLUS => v2 + v1,
                    MINUS => v2 - v1,
                    MUL => v2 * v1,
                    _ => unreachable!(),
                };
                stack.push(T { v: result, id });
            }
        }

        if f {
            ans = stack.pop().v;
            f = false;
        } else if ans < stack.pop().v {
            ans = stack.data[stack.top - 1].v;
        }
    }

    println!("{}", ans);
}
