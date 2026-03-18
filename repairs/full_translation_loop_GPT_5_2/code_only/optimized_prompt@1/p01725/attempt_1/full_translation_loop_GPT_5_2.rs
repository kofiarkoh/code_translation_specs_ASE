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
        _ => MUL, // includes '*'
    }
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

fn get_int(bytes: &[u8], p: &mut usize) -> i64 {
    let mut n: i64 = 0;
    while *p < bytes.len() && is_digit(bytes[*p]) {
        // C uses signed long long; overflow is UB there.
        // We use wrapping to emulate typical 2's-complement behavior.
        n = n.wrapping_mul(10).wrapping_add((bytes[*p] - b'0') as i64);
        *p += 1;
    }
    n
}

fn rank(tbl: &[[i32; 3]; 13], rid: usize, op: i8) -> i32 {
    if op == LEFT {
        LEFT as i32
    } else {
        tbl[rid][op as usize]
    }
}

fn main() {
    // Read entire stdin, then mimic scanf("%s", str): take first whitespace-delimited token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = match input.split_whitespace().next() {
        Some(t) => t,
        None => return,
    };
    let bytes = s.as_bytes();

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

    let mut stack: [T; MAX + 3] = [T::default(); MAX + 3];
    let mut buf: [T; MAX + 3] = [T::default(); MAX + 3];

    let mut f = true;
    let mut ans: i64 = 0;

    for rid in 0..13 {
        let mut p: usize = 0;
        let mut top: usize = 0;
        let mut end: usize = 0;

        // Shunting-yard to RPN with custom precedence table.
        while p < bytes.len() {
            let c = bytes[p];
            if is_digit(c) {
                buf[end].id = VAL;
                buf[end].v = get_int(bytes, &mut p);
                end += 1;
            } else if c == b')' {
                while top > 0 && stack[top - 1].id != LEFT {
                    top -= 1;
                    buf[end] = stack[top];
                    end += 1;
                }
                p += 1;
                // In the C code, this decrements even if no LEFT exists (would underflow UB).
                // We guard to avoid panic; behavior for invalid expressions is unspecified anyway.
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
                    if rank(&tbl, rid, id) <= rank(&tbl, rid, stack[top - 1].id) {
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
                // C assumes well-formed expression; avoid panic on malformed input.
                if top < 2 {
                    top = 0;
                    break;
                }
                top -= 1;
                let v1 = stack[top].v;
                top -= 1;
                let mut v2 = stack[top].v;

                // Use wrapping to emulate typical 2's-complement behavior (C signed overflow is UB).
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

        if top == 0 {
            continue;
        }
        top -= 1;
        let val = stack[top].v;

        if f {
            ans = val;
            f = false;
        } else if ans < val {
            ans = val;
        }
    }

    print!("{ans}\n");
}
