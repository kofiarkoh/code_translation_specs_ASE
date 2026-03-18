use std::io::{self, Read};

#[derive(Clone, Debug)]
enum Tok {
    Num(i64),
    Op(char), // '+', '-', '*'
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Vec<Tok> {
    let b = s.as_bytes();
    let mut i = 0usize;
    let mut out = Vec::new();

    while i < b.len() {
        let c = b[i] as char;
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() {
            let mut v: i64 = 0;
            while i < b.len() && (b[i] as char).is_ascii_digit() {
                v = v
                    .wrapping_mul(10)
                    .wrapping_add((b[i] - b'0') as i64);
                i += 1;
            }
            out.push(Tok::Num(v));
            continue;
        }
        match c {
            '+' | '-' | '*' => out.push(Tok::Op(c)),
            '(' => out.push(Tok::LParen),
            ')' => out.push(Tok::RParen),
            _ => {
                // Input is specified valid; ignore anything else narrowly.
            }
        }
        i += 1;
    }
    out
}

fn apply_op(a: i64, b: i64, op: char) -> i64 {
    match op {
        '+' => a.wrapping_add(b),
        '-' => a.wrapping_sub(b),
        '*' => a.wrapping_mul(b),
        _ => 0,
    }
}

fn eval_with_prec(tokens: &[Tok], prec: &[i32; 3]) -> i64 {
    // Shunting-yard to RPN, then evaluate.
    // Here, smaller numeric value means higher precedence (evaluated earlier),
    // matching the common convention and the sample's expected behavior.
    let mut out: Vec<Tok> = Vec::new();
    let mut ops: Vec<Tok> = Vec::new();

    let prec_of = |op: char| -> i32 {
        match op {
            '+' => prec[0],
            '-' => prec[1],
            '*' => prec[2],
            _ => i32::MAX,
        }
    };

    for t in tokens.iter().cloned() {
        match t {
            Tok::Num(_) => out.push(t),
            Tok::Op(op) => {
                let p = prec_of(op);
                while let Some(top) = ops.last() {
                    match top {
                        Tok::Op(top_op) => {
                            let tp = prec_of(*top_op);
                            // Left-associative: pop while top precedence <= current precedence
                            // (because smaller number => higher precedence).
                            if tp <= p {
                                out.push(ops.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        Tok::LParen => break,
                        _ => break,
                    }
                }
                ops.push(Tok::Op(op));
            }
            Tok::LParen => ops.push(Tok::LParen),
            Tok::RParen => {
                while let Some(top) = ops.pop() {
                    match top {
                        Tok::LParen => break,
                        Tok::Op(_) => out.push(top),
                        _ => {}
                    }
                }
            }
        }
    }
    while let Some(top) = ops.pop() {
        if matches!(top, Tok::Op(_)) {
            out.push(top);
        }
    }

    let mut st: Vec<i64> = Vec::new();
    for t in out {
        match t {
            Tok::Num(v) => st.push(v),
            Tok::Op(op) => {
                let b = st.pop().unwrap_or(0);
                let a = st.pop().unwrap_or(0);
                st.push(apply_op(a, b, op));
            }
            _ => {}
        }
    }
    st.pop().unwrap_or(0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let expr = input.trim();
    let tokens = tokenize(expr);

    // All strict precedence orders among '+', '-', '*': 3! = 6.
    // Value 0 is highest precedence, 2 is lowest.
    let perms: [[i32; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let mut best: Option<i64> = None;
    for p in perms.iter() {
        let v = eval_with_prec(&tokens, p);
        best = Some(match best {
            None => v,
            Some(b) => if v < b { v } else { b },
        });
    }

    print!("{}", best.unwrap_or(0));
}
