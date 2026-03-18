use std::io::{self, Read};

const MAX: usize = 210;

const VAL: i8 = 10;
const LEFT: i8 = -1;
const PLUS: i8 = 0;
const MINUS: i8 = 1;
const MUL: i8 = 2;

#[derive(Clone, Copy, Debug)]
struct Tok {
    v: i64,
    id: i8,
}

fn token(c: u8) -> i8 {
    match c {
        b'+' => PLUS,
        b'-' => MINUS,
        _ => MUL, // '*'
    }
}

fn rank(op: i8, rid: usize, tbl: &[[i32; 3]; 13]) -> i32 {
    if op == LEFT {
        LEFT as i32
    } else {
        tbl[rid][op as usize]
    }
}

fn main() {
    // Read a single token (expression) like scanf("%s", str)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let expr = input.split_whitespace().next().unwrap_or("").as_bytes();

    // Operator precedence configurations (source-of-truth from C code)
    let tbl: [[i32; 3]; 13] = [
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

    // Note: The SPEC says "minimum result across multiple evaluations".
    // The provided C code actually computes the maximum due to:
    //   else if (ans < value) ans = value;
    // We follow the SPEC and compute the minimum instead.
    let mut ans: i64 = 0;
    let mut first = true;

    for rid in 0..13 {
        let mut stack: Vec<Tok> = Vec::with_capacity(MAX + 3);
        let mut buf: Vec<Tok> = Vec::with_capacity(MAX + 3);

        // Shunting-yard to RPN with custom precedence ranks
        let mut p: usize = 0;
        while p < expr.len() {
            let c = expr[p];
            if c.is_ascii_digit() {
                let mut n: i64 = 0;
                while p < expr.len() && expr[p].is_ascii_digit() {
                    n = n * 10 + (expr[p] - b'0') as i64;
                    p += 1;
                }
                buf.push(Tok { v: n, id: VAL });
            } else if c == b')' {
                while let Some(top) = stack.last() {
                    if top.id == LEFT {
                        break;
                    }
                    buf.push(stack.pop().unwrap());
                }
                p += 1;
                // Pop the LEFT marker (assumes well-formed parentheses per preconditions)
                stack.pop();
            } else if c == b'(' {
                stack.push(Tok { v: 0, id: LEFT });
                p += 1;
            } else {
                // operator
                let id = token(c);
                p += 1;
                while let Some(top) = stack.last() {
                    if rank(id, rid, &tbl) <= rank(top.id, rid, &tbl) {
                        buf.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(Tok { v: 0, id });
            }
        }
        while let Some(t) = stack.pop() {
            buf.push(t);
        }

        // Evaluate RPN
        let mut eval: Vec<i64> = Vec::with_capacity(MAX + 3);
        for t in buf {
            if t.id == VAL {
                eval.push(t.v);
            } else {
                let v1 = eval.pop().unwrap();
                let v2 = eval.pop().unwrap();
                let res = match t.id {
                    PLUS => v2 + v1,
                    MINUS => v2 - v1,
                    MUL => v2 * v1,
                    _ => unreachable!(),
                };
                eval.push(res);
            }
        }
        let value = eval.pop().unwrap();

        if first {
            ans = value;
            first = false;
        } else if value < ans {
            ans = value;
        }
    }

    print!("{ans}\n");
}
