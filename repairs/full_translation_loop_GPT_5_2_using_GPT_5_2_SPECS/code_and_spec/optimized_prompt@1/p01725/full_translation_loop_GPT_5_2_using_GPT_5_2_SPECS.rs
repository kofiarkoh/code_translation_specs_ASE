use std::io::{self, Read};

const MAX: usize = 210;

const VAL: i8 = 10;
const LEFT: i8 = -1;
const PLUS: i8 = 0;
const MINUS: i8 = 1;
const MUL: i8 = 2;

// 13 precedence/associativity schemes (as in the C code).
const TBL: [[i32; 3]; 13] = [
    [1, 2, 3],
    [1, 3, 2],
    [2, 1, 3],
    [2, 3, 1],
    [3, 1, 2],
    [3, 2, 1],
    [1, 1, 1],
    [1, 1, 2],
    [2, 2, 1],
    [1, 2, 1],
    [2, 1, 2],
    [1, 2, 2],
    [2, 1, 1],
];

#[derive(Clone, Copy, Default)]
struct Tok {
    v: i64,
    id: i8,
}

fn token(c: u8) -> i8 {
    match c {
        b'+' => PLUS,
        b'-' => MINUS,
        _ => MUL, // only '*'
    }
}

fn rank(rid: usize, op: i8) -> i32 {
    if op == LEFT {
        LEFT as i32
    } else {
        TBL[rid][op as usize]
    }
}

fn main() {
    // Read one token (expression) from stdin, like scanf("%s", str).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let expr = input.split_whitespace().next().unwrap_or("").as_bytes();

    // Workspace sized similarly to the C code.
    let mut stack: [Tok; MAX + 3] = [Tok::default(); MAX + 3];
    let mut buf: [Tok; MAX + 3] = [Tok::default(); MAX + 3];

    let mut first = true;
    let mut ans: i64 = 0;

    for rid in 0..13 {
        let mut p: usize = 0;
        let mut top: usize = 0;
        let mut end: usize = 0;

        // Shunting-yard to RPN, with scheme-dependent precedence.
        while p < expr.len() {
            let c = expr[p];
            if c.is_ascii_digit() {
                let mut n: i64 = 0;
                while p < expr.len() && expr[p].is_ascii_digit() {
                    n = n * 10 + (expr[p] - b'0') as i64;
                    p += 1;
                }
                buf[end] = Tok { v: n, id: VAL };
                end += 1;
            } else if c == b')' {
                while top > 0 && stack[top - 1].id != LEFT {
                    top -= 1;
                    buf[end] = stack[top];
                    end += 1;
                }
                p += 1;
                // Pop the LEFT marker (assumes valid input).
                top -= 1;
            } else if c == b'(' {
                stack[top] = Tok { v: 0, id: LEFT };
                top += 1;
                p += 1;
            } else {
                let id = token(c);
                p += 1;
                while top > 0 {
                    if rank(rid, id) <= rank(rid, stack[top - 1].id) {
                        top -= 1;
                        buf[end] = stack[top];
                        end += 1;
                    } else {
                        break;
                    }
                }
                stack[top] = Tok { v: 0, id };
                top += 1;
            }
        }
        while top > 0 {
            top -= 1;
            buf[end] = stack[top];
            end += 1;
        }

        // Evaluate RPN.
        top = 0;
        for i in 0..end {
            let id = buf[i].id;
            if id == VAL {
                stack[top].v = buf[i].v;
                top += 1;
            } else {
                // Assumes valid input (no underflow).
                top -= 1;
                let v1 = stack[top].v;
                top -= 1;
                let mut v2 = stack[top].v;
                match id {
                    PLUS => v2 += v1,
                    MINUS => v2 -= v1,
                    MUL => v2 *= v1,
                    _ => {}
                }
                stack[top].v = v2;
                top += 1;
            }
        }

        // Result is on stack[0].
        top -= 1;
        let res = stack[top].v;

        if first {
            ans = res;
            first = false;
        } else if ans < res {
            ans = res;
        }
    }

    print!("{ans}\n");
}
