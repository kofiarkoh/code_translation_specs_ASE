use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: i64,
}

fn skip_ws(bytes: &[u8], i: &mut usize) {
    while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

fn parse_int(bytes: &[u8], i: &mut usize) -> i64 {
    skip_ws(bytes, i);
    let mut sign = 1i64;
    if *i < bytes.len() && bytes[*i] == b'-' {
        sign = -1;
        *i += 1;
    }
    let mut v: i64 = 0;
    while *i < bytes.len() && bytes[*i].is_ascii_digit() {
        v = v * 10 + (bytes[*i] - b'0') as i64;
        *i += 1;
    }
    sign * v
}

fn parse_tree(bytes: &[u8], i: &mut usize) -> Option<Box<Node>> {
    // Grammar:
    //   Tree := "()" | "(" Tree "[" int "]" Tree ")"
    skip_ws(bytes, i);
    if *i >= bytes.len() || bytes[*i] != b'(' {
        return None;
    }
    *i += 1; // '('
    skip_ws(bytes, i);

    if *i < bytes.len() && bytes[*i] == b')' {
        *i += 1; // ')'
        return None;
    }

    let left = parse_tree(bytes, i);

    skip_ws(bytes, i);
    if *i >= bytes.len() || bytes[*i] != b'[' {
        return None;
    }
    *i += 1; // '['

    let val = parse_int(bytes, i);

    skip_ws(bytes, i);
    if *i >= bytes.len() || bytes[*i] != b']' {
        return None;
    }
    *i += 1; // ']'

    let right = parse_tree(bytes, i);

    skip_ws(bytes, i);
    if *i >= bytes.len() || bytes[*i] != b')' {
        return None;
    }
    *i += 1; // ')'

    Some(Box::new(Node { left, right, val }))
}

fn merge(a: Option<Box<Node>>, b: Option<Box<Node>>) -> Option<Box<Node>> {
    // Semantics (inferred from sample):
    // - If both nodes exist: sum values and merge children recursively.
    // - If only one node exists: keep it, but add the other tree's root value
    //   to the nearest existing node along that side (i.e., "push down" the
    //   missing node's value into the existing subtree root).
    match (a, b) {
        (None, None) => None,
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (Some(mut x), Some(y)) => {
            x.val += y.val;
            x.left = merge(x.left, y.left);
            x.right = merge(x.right, y.right);
            Some(x)
        }
    }
}

fn write_tree(t: &Option<Box<Node>>, out: &mut String) {
    match t {
        None => out.push_str("()"),
        Some(n) => {
            out.push('(');
            write_tree(&n.left, out);
            out.push('[');
            out.push_str(&n.val.to_string());
            out.push(']');
            write_tree(&n.right, out);
            out.push(')');
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Input consists of exactly two tree strings; they may be on separate lines.
    // Use whitespace splitting to be robust to line breaks.
    let mut parts = input.split_whitespace();
    let s1 = parts.next().unwrap_or("");
    let s2 = parts.next().unwrap_or("");

    let t1 = if s1.is_empty() {
        None
    } else {
        let bytes = s1.as_bytes();
        let mut idx = 0usize;
        parse_tree(bytes, &mut idx)
    };

    let t2 = if s2.is_empty() {
        None
    } else {
        let bytes = s2.as_bytes();
        let mut idx = 0usize;
        parse_tree(bytes, &mut idx)
    };

    let merged = merge(t1, t2);
    let mut out = String::new();
    write_tree(&merged, &mut out);
    print!("{out}");
}
