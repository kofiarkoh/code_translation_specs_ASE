use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct MyStr<'a> {
    s: &'a [u8],
    len: usize,
}

impl<'a> MyStr<'a> {
    fn new(s: &'a [u8]) -> Self {
        Self { s, len: s.len() }
    }
    fn slice(&self, start: usize, len: usize) -> Self {
        Self {
            s: &self.s[start..start + len],
            len,
        }
    }
}

/// Parse an integer starting at `start` until a non-digit (or end).
/// Matches C's `atoi` behavior for the expected valid inputs (non-negative integers).
fn parse_atoi(bytes: &[u8], start: usize) -> i32 {
    let mut i = start;
    let mut val: i32 = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if (b'0'..=b'9').contains(&c) {
            val = val * 10 + (c - b'0') as i32;
            i += 1;
        } else {
            break;
        }
    }
    val
}

/// Analyze a tree string `in_` into left subtree (including its surrounding parens),
/// right subtree (including its surrounding parens), and node value.
/// This follows the C logic: it tracks nesting level across both () and [].
fn analyze_tree<'a>(in_: MyStr<'a>) -> (MyStr<'a>, MyStr<'a>, i32) {
    let p = in_.s;
    let mut left = MyStr { s: p, len: 0 };
    let mut right = MyStr { s: &p[in_.len..in_.len], len: 0 };
    let mut val: i32 = 0;

    let mut lv: i32 = 0;
    for i in 0..in_.len {
        match p[i] {
            b'[' => {
                if lv == 0 {
                    left.len = i;
                    val = parse_atoi(p, i + 1);
                }
                lv += 1;
            }
            b']' => {
                if lv == 1 {
                    // right starts after this ']'
                    let start = i + 1;
                    right = MyStr {
                        s: &p[start..in_.len],
                        len: in_.len - start,
                    };
                }
                lv -= 1;
            }
            b'(' => lv += 1,
            b')' => lv -= 1,
            _ => {}
        }
    }

    (left, right, val)
}

/// In the original C, an "empty tree" is exactly length 2, i.e. "()".
fn is_empty_tree(in_: MyStr<'_>) -> bool {
    in_.len == 2
}

/// Remove the outer parentheses: "(...)" -> "..."
fn remove_paren<'a>(in_: MyStr<'a>) -> MyStr<'a> {
    // Preconditions (from problem statement): valid rooted tree structures.
    // So len >= 2 and first/last are '(' and ')'.
    in_.slice(1, in_.len - 2)
}

/// Combine two trees and return the resulting tree string.
fn add_tree(in1: MyStr<'_>, in2: MyStr<'_>) -> String {
    let (mut l1, mut r1, v1) = analyze_tree(in1);
    let (mut l2, mut r2, v2) = analyze_tree(in2);
    let val = v1 + v2;

    let left_str = if is_empty_tree(l1) || is_empty_tree(l2) {
        String::new()
    } else {
        l1 = remove_paren(l1);
        l2 = remove_paren(l2);
        add_tree(l1, l2)
    };

    let right_str = if is_empty_tree(r1) || is_empty_tree(r2) {
        String::new()
    } else {
        r1 = remove_paren(r1);
        r2 = remove_paren(r2);
        add_tree(r1, r2)
    };

    // Match C's formatting: "(%.*s)[%d](%.*s)"
    format!("({})[{}]({})", left_str, val, right_str)
}

fn main() {
    // Read two whitespace-separated strings (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Spec says: if either input tree is empty, treat as null and omit subtree.
    // The C code assumes both exist; we follow spec by defaulting missing to empty.
    let t1 = it.next().unwrap_or("");
    let t2 = it.next().unwrap_or("");

    // If an input is the empty string, treat it as a null tree.
    // The C representation of an empty tree is "()", so map "" -> "()".
    let t1 = if t1.is_empty() { "()" } else { t1 };
    let t2 = if t2.is_empty() { "()" } else { t2 };

    let b1 = t1.as_bytes();
    let b2 = t2.as_bytes();

    let out = add_tree(MyStr::new(b1), MyStr::new(b2));
    print!("{out}\n");
}
