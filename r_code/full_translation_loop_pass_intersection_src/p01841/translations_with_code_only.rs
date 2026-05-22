use std::io::{self, Read, Write};

#[derive(Clone, Copy, Debug)]
struct MyStr {
    start: usize,
    len: usize,
}

fn is_empty_tree(s: &MyStr) -> bool {
    // C: if(in->len==2) return(1);
    s.len == 2
}

fn remove_paren(s: &mut MyStr) {
    // C: in->len -= 2; in->sptr++;
    s.len = s.len.saturating_sub(2);
    s.start += 1;
}

fn analyze_tree(buf: &[u8], input: &MyStr) -> (MyStr, MyStr, i32) {
    // Returns (left, right, val) as in the C code.
    let p0 = input.start;
    let mut left = MyStr {
        start: p0,
        len: input.len,
    };
    let mut right = MyStr { start: p0, len: 0 };
    let mut val: i32 = 0;

    let mut lv: i32 = 0;
    for i in 0..input.len {
        let c = buf[p0 + i];
        if c == b'[' {
            if lv == 0 {
                left.len = i;

                // C: *val = atoi(p+i+1);
                // Implement a minimal atoi-like parse (no leading whitespace expected here).
                let mut j = p0 + i + 1;
                let end = p0 + input.len;

                let mut sign: i32 = 1;
                if j < end && buf[j] == b'-' {
                    sign = -1;
                    j += 1;
                } else if j < end && buf[j] == b'+' {
                    j += 1;
                }

                let mut acc: i32 = 0;
                while j < end {
                    let d = buf[j];
                    if !(b'0'..=b'9').contains(&d) {
                        break;
                    }
                    // C's atoi overflow is UB; use wrapping to emulate typical behavior.
                    acc = acc.wrapping_mul(10).wrapping_add((d - b'0') as i32);
                    j += 1;
                }
                val = acc.wrapping_mul(sign);
            }
            lv += 1;
        } else if c == b']' {
            if lv == 1 {
                right.start = p0 + i + 1;
                right.len = input.len - i - 1;
            }
            lv -= 1;
        } else if c == b'(' {
            lv += 1;
        } else if c == b')' {
            lv -= 1;
        }
    }

    (left, right, val)
}

fn add_tree(buf1: &[u8], in1: MyStr, buf2: &[u8], in2: MyStr, out_buf: &mut Vec<u8>) -> MyStr {
    let (mut l1, mut r1, v1) = analyze_tree(buf1, &in1);
    let (mut l2, mut r2, v2) = analyze_tree(buf2, &in2);
    let val = v1.wrapping_add(v2);

    // Left subtree
    let left_sub = if is_empty_tree(&l1) || is_empty_tree(&l2) {
        MyStr { start: 0, len: 0 }
    } else {
        remove_paren(&mut l1);
        remove_paren(&mut l2);
        add_tree(buf1, l1, buf2, l2, out_buf)
    };

    // Right subtree
    let right_sub = if is_empty_tree(&r1) || is_empty_tree(&r2) {
        MyStr { start: 0, len: 0 }
    } else {
        remove_paren(&mut r1);
        remove_paren(&mut r2);
        add_tree(buf1, r1, buf2, r2, out_buf)
    };

    // Emit "(%.*s)[%d](%.*s)" into out_buf.
    // Need to avoid aliasing immutable and mutable borrows of out_buf; copy slices out first.
    let left_bytes: Vec<u8> = if left_sub.len == 0 {
        Vec::new()
    } else {
        out_buf[left_sub.start..left_sub.start + left_sub.len].to_vec()
    };
    let right_bytes: Vec<u8> = if right_sub.len == 0 {
        Vec::new()
    } else {
        out_buf[right_sub.start..right_sub.start + right_sub.len].to_vec()
    };

    let out_start = out_buf.len();
    out_buf.push(b'(');
    out_buf.extend_from_slice(&left_bytes);
    out_buf.push(b')');
    out_buf.push(b'[');
    out_buf.extend_from_slice(val.to_string().as_bytes());
    out_buf.push(b']');
    out_buf.push(b'(');
    out_buf.extend_from_slice(&right_bytes);
    out_buf.push(b')');

    MyStr {
        start: out_start,
        len: out_buf.len() - out_start,
    }
}

fn main() {
    // Mimic scanf("%s") twice: read two whitespace-delimited tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let s1 = it.next().unwrap_or("");
    let s2 = it.next().unwrap_or("");

    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();

    let t1 = MyStr { start: 0, len: b1.len() };
    let t2 = MyStr { start: 0, len: b2.len() };

    let mut ans: Vec<u8> = Vec::with_capacity(100000);
    let res = add_tree(b1, t1, b2, t2, &mut ans);

    // display(): print characters then '\n'
    let out = &ans[res.start..res.start + res.len];
    let mut stdout = io::stdout().lock();
    stdout.write_all(out).unwrap();
    stdout.write_all(b"\n").unwrap();
}
