use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Slice<'a> {
    s: &'a str,
}

impl<'a> Slice<'a> {
    fn len(&self) -> usize {
        self.s.len()
    }
}

/// Parse a tree string of the form: (LEFT)[VAL](RIGHT)
/// Returns (left_subtree_str_including_parens, val, right_subtree_str_including_parens)
///
/// This mirrors the C `analyze_tree` logic:
/// - `left` is everything before the first '[' at nesting level 0
/// - `val` is parsed starting right after that '['
/// - `right` starts right after the matching ']' that closes that '[' (i.e., when level goes from 1 to 0)
fn analyze_tree<'a>(input: Slice<'a>) -> (Slice<'a>, i32, Slice<'a>) {
    let bytes = input.s.as_bytes();
    let mut lv: i32 = 0;

    let mut left_len: Option<usize> = None;
    let mut val: i32 = 0;
    let mut right_start: Option<usize> = None;

    let n = bytes.len();
    let mut i = 0usize;
    while i < n {
        match bytes[i] {
            b'[' => {
                if lv == 0 {
                    left_len = Some(i);
                    // Parse integer starting at i+1, like C's atoi (stops at first non-digit after optional sign).
                    let mut j = i + 1;
                    // optional sign
                    let mut sign: i32 = 1;
                    if j < n && bytes[j] == b'-' {
                        sign = -1;
                        j += 1;
                    } else if j < n && bytes[j] == b'+' {
                        j += 1;
                    }
                    let mut acc: i32 = 0;
                    while j < n {
                        let c = bytes[j];
                        if (b'0'..=b'9').contains(&c) {
                            acc = acc.saturating_mul(10).saturating_add((c - b'0') as i32);
                            j += 1;
                        } else {
                            break;
                        }
                    }
                    val = acc.saturating_mul(sign);
                }
                lv += 1;
            }
            b']' => {
                if lv == 1 {
                    right_start = Some(i + 1);
                }
                lv -= 1;
            }
            b'(' => lv += 1,
            b')' => lv -= 1,
            _ => {}
        }
        i += 1;
    }

    let left_len = left_len.expect("well-formed input per spec");
    let right_start = right_start.expect("well-formed input per spec");

    let left = Slice { s: &input.s[..left_len] };
    let right = Slice { s: &input.s[right_start..] };
    (left, val, right)
}

/// In the original C, an "empty tree" is exactly length 2, i.e. "()".
fn is_empty_tree(s: Slice<'_>) -> bool {
    s.len() == 2
}

/// Remove the outer parentheses: "(...)" -> "..."
/// Mirrors C `remove_paren` which does: len -= 2; sptr++.
fn remove_paren<'a>(s: Slice<'a>) -> Slice<'a> {
    // Per spec, inputs are well-formed; this assumes s is at least "()" and starts/ends with parens.
    let bytes = s.s.as_bytes();
    debug_assert!(bytes.len() >= 2 && bytes[0] == b'(' && bytes[bytes.len() - 1] == b')');
    Slice { s: &s.s[1..s.s.len() - 1] }
}

/// Combine two trees per spec/C behavior and return the resulting tree string.
fn add_tree(in1: Slice<'_>, in2: Slice<'_>) -> String {
    let (l1, v1, r1) = analyze_tree(in1);
    let (l2, v2, r2) = analyze_tree(in2);
    let val = v1 + v2;

    let left_out = if is_empty_tree(l1) || is_empty_tree(l2) {
        String::new()
    } else {
        let l1i = remove_paren(l1);
        let l2i = remove_paren(l2);
        add_tree(l1i, l2i)
    };

    let right_out = if is_empty_tree(r1) || is_empty_tree(r2) {
        String::new()
    } else {
        let r1i = remove_paren(r1);
        let r2i = remove_paren(r2);
        add_tree(r1i, r2i)
    };

    // Exactly matches C formatting: "(%s)[%d](%s)" where %s may be empty.
    format!("({})[{}]({})", left_out, val, right_out)
}

fn main() {
    // Read two whitespace-delimited strings.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let t1 = it.next().unwrap_or("");
    let t2 = it.next().unwrap_or("");

    let out = add_tree(Slice { s: t1 }, Slice { s: t2 });
    print!("{out}\n");
}
