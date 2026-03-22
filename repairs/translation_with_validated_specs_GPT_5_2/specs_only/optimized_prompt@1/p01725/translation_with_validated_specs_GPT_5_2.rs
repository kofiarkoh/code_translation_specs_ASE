use std::io::{self, Read};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
}

#[derive(Clone, Debug)]
enum Tok {
    Num(i64),
    Op(Op),
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Vec<Tok> {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut out = Vec::new();

    while i < bytes.len() {
        match bytes[i] {
            b'0'..=b'9' => {
                let mut v: i64 = 0;
                while i < bytes.len() {
                    match bytes[i] {
                        b'0'..=b'9' => {
                            v = v * 10 + (bytes[i] - b'0') as i64;
                            i += 1;
                        }
                        _ => break,
                    }
                }
                out.push(Tok::Num(v));
            }
            b'+' => {
                out.push(Tok::Op(Op::Add));
                i += 1;
            }
            b'-' => {
                out.push(Tok::Op(Op::Sub));
                i += 1;
            }
            b'*' => {
                out.push(Tok::Op(Op::Mul));
                i += 1;
            }
            b'(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            b')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            _ => {
                // Undefined behavior per spec for unexpected characters; we choose to stop.
                // This keeps code safe/compilable without inventing recovery.
                break;
            }
        }
    }

    out
}

fn apply_op(a: i64, op: Op, b: i64) -> i64 {
    match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
    }
}

fn precedence_of(op: Op, prec: &[i32; 3]) -> i32 {
    match op {
        Op::Add => prec[0],
        Op::Sub => prec[1],
        Op::Mul => prec[2],
    }
}

fn is_right_assoc(op: Op, right_assoc: &[bool; 3]) -> bool {
    match op {
        Op::Add => right_assoc[0],
        Op::Sub => right_assoc[1],
        Op::Mul => right_assoc[2],
    }
}

fn eval_with_scheme(tokens: &[Tok], prec: &[i32; 3], right_assoc: &[bool; 3]) -> i64 {
    // Shunting-yard to RPN, with parentheses always enforced.
    let mut out: Vec<Tok> = Vec::new();
    let mut ops: Vec<Tok> = Vec::new();

    for t in tokens.iter().cloned() {
        match t {
            Tok::Num(_) => out.push(t),
            Tok::Op(op1) => {
                let p1 = precedence_of(op1, prec);
                let ra1 = is_right_assoc(op1, right_assoc);

                while let Some(top) = ops.last().cloned() {
                    match top {
                        Tok::Op(op2) => {
                            let p2 = precedence_of(op2, prec);
                            // If op1 is left-assoc: pop while p2 >= p1
                            // If op1 is right-assoc: pop while p2 > p1
                            let should_pop = if ra1 { p2 > p1 } else { p2 >= p1 };
                            if should_pop {
                                out.push(ops.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        Tok::LParen => break,
                        _ => break,
                    }
                }
                ops.push(Tok::Op(op1));
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
        match top {
            Tok::Op(_) => out.push(top),
            _ => {}
        }
    }

    // Evaluate RPN.
    let mut st: Vec<i64> = Vec::new();
    for t in out {
        match t {
            Tok::Num(v) => st.push(v),
            Tok::Op(op) => {
                // Preconditions guarantee well-formedness; otherwise undefined.
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                st.push(apply_op(a, op, b));
            }
            _ => {}
        }
    }
    st.pop().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let expr = input.split_whitespace().next().unwrap_or("");
    let tokens = tokenize(expr);

    // 13 fixed precedence/associativity schemes for {+, -, *}.
    // We interpret "precedence/associativity schemes" narrowly as:
    // - all 6 strict precedence orders (3!),
    // - plus 7 tie-group schemes where some operators share precedence,
    //   with left-to-right associativity for tied operators.
    //
    // This yields exactly 13 schemes:
    //   6 (all distinct) + 6 (exactly two tied) + 1 (all tied) = 13.
    //
    // For strict orders, associativity is irrelevant; we keep all left-assoc.
    // For tie schemes, we also keep left-assoc (narrowest deterministic choice).
    let schemes: Vec<([i32; 3], [bool; 3])> = vec![
        // 6 strict precedence orders (higher number = higher precedence)
        ([1, 2, 3], [false, false, false]), // + < - < *
        ([1, 3, 2], [false, false, false]), // + < * < -
        ([2, 1, 3], [false, false, false]), // - < + < *
        ([2, 3, 1], [false, false, false]), // - < * < +
        ([3, 1, 2], [false, false, false]), // * < + < -
        ([3, 2, 1], [false, false, false]), // * < - < +

        // 6 schemes where exactly two operators tie (left-assoc within tie)
        ([1, 1, 2], [false, false, false]), // + = - < *
        ([1, 2, 1], [false, false, false]), // + = * < -
        ([2, 1, 1], [false, false, false]), // - = * < +

        ([2, 2, 1], [false, false, false]), // * < + = -
        ([2, 1, 2], [false, false, false]), // - < + = *
        ([1, 2, 2], [false, false, false]), // + < - = *

        // 1 scheme where all tie (pure left-to-right, parentheses enforced)
        ([1, 1, 1], [false, false, false]),
    ];

    let mut best: i64 = i64::MIN;
    for (prec, assoc) in schemes {
        let v = eval_with_scheme(&tokens, &prec, &assoc);
        if v > best {
            best = v;
        }
    }

    print!("{}", best);
}
