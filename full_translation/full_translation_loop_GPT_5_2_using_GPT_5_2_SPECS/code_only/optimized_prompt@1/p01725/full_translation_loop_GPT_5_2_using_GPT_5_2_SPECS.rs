use std::io::{self, Read};

const MAX: usize = 210;

const VAL: i8 = 10;
const LEFT: i8 = -1;
const PLUS: i8 = 0;
const MINUS: i8 = 1;
const MUL: i8 = 2;

#[derive(Copy, Clone, Default)]
struct T {
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

fn rank(op: i8, rid: usize, tbl: &[[i32; 3]; 13]) -> i32 {
    if op == LEFT {
        LEFT as i32
    } else {
        tbl[rid][op as usize]
    }
}

fn is_digit(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}

fn main() {
    // Read entire stdin, then mimic scanf("%s", str): take first whitespace-delimited token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = match input.split_whitespace().next() {
        Some(t) => t.as_bytes().to_vec(),
        None => return, // no input; C would have undefined behavior, but we just exit.
    };

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

    let mut stack = [T::default(); MAX + 3];
    let mut buf = [T::default(); MAX + 3];

    let mut first = true;
    let mut ans: i64 = 0;

    for rid in 0..13 {
        let mut p: usize = 0;
        let mut top: usize = 0;
        let mut end: usize = 0;

        // Shunting-yard to RPN, with custom precedence per rid.
        while p < s.len() {
            let c = s[p];
            if is_digit(c) {
                // getInt(): parse consecutive digits, using i64 wrapping like C signed overflow intent.
                let mut n: i64 = 0;
                while p < s.len() && is_digit(s[p]) {
                    let d = (s[p] - b'0') as i64;
                    n = n.wrapping_mul(10).wrapping_add(d);
                    p += 1;
                }
                buf[end].id = VAL;
                buf[end].v = n;
                end += 1;
            } else if c == b')' {
                while top > 0 && stack[top - 1].id != LEFT {
                    top -= 1;
                    buf[end] = stack[top];
                    end += 1;
                }
                p += 1;
                // In the C code, this does `top--` unconditionally (assumes matching '(').
                // Preserve behavior: if malformed, this would underflow in C (UB); here we saturate at 0.
                if top > 0 {
                    top -= 1;
                }
            } else if c == b'(' {
                stack[top].id = LEFT;
                top += 1;
                p += 1;
            } else {
                let id = token(c);
                p += 1;
                while top > 0 {
                    let cond = rank(id, rid, &tbl) <= rank(stack[top - 1].id, rid, &tbl);
                    if cond {
                        top -= 1;
                        buf[end] = stack[top];
                        end += 1;
                    } else {
                        break;
                    }
                }
                stack[top].id = id;
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
                // C assumes enough operands; preserve wrapping arithmetic.
                top -= 1;
                let v1 = stack[top].v;
                top -= 1;
                let mut v2 = stack[top].v;
                if id == PLUS {
                    v2 = v2.wrapping_add(v1);
                } else if id == MINUS {
                    v2 = v2.wrapping_sub(v1);
                } else if id == MUL {
                    v2 = v2.wrapping_mul(v1);
                }
                stack[top].v = v2;
                top += 1;
            }
        }

        // Replicate the (slightly odd) C logic exactly.
        if first {
            top -= 1;
            ans = stack[top].v;
            first = false;
        } else {
            top -= 1;
            if ans < stack[top].v {
                // C does: ans = stack[top].v; (after having decremented top in the condition)
                ans = stack[top].v;
            }
        }
    }

    print!("{}\n", ans);
}
