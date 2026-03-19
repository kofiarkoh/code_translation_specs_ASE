/*
A2740
title:Rooted Tree for Misawa-san
@kankichi573
*/

use std::io::{self, Write};
use std::str;

struct MyStr<'a> {
    sptr: &'a str,
    len: usize,
}

fn display(str_: &MyStr) {
    print!("{}", str_.sptr);
    #[cfg(debug_assertions)]
    {
        print!("{{{}}}", str_.len);
    }
    println!();
}

fn analyze_tree(in_: &MyStr, left: &mut MyStr, right: &mut MyStr, val: &mut i32) {
    let p = in_.sptr;
    let mut lv = 0;

    left.sptr = p;

    for (i, c) in p.chars().enumerate() {
        match c {
            '[' => {
                if lv == 0 {
                    left.len = i;
                    *val = p[i + 1..].parse().unwrap();
                }
                lv += 1;
            }
            ']' => {
                if lv == 1 {
                    right.sptr = &p[i + 1..];
                    right.len = in_.len - i - 1;
                }
                lv -= 1;
            }
            '(' => lv += 1,
            ')' => lv -= 1,
            _ => {}
        }
    }
}

fn is_empty_tree(in_: &MyStr) -> bool {
    in_.len == 2
}

fn remove_paren(in_: &mut MyStr) {
    in_.len -= 2;
    in_.sptr = &in_.sptr[1..];
}

fn add_tree(in1: &MyStr, in2: &MyStr, out: &mut MyStr) {
    let mut stree = [[MyStr { sptr: "", len: 0 }; 2]; 2];
    let mut sstree = [MyStr { sptr: "", len: 0 }; 2];
    let mut v = [0; 2];
    let mut val;

    analyze_tree(in1, &mut stree[0][0], &mut stree[0][1], &mut v[0]);
    analyze_tree(in2, &mut stree[1][0], &mut stree[1][1], &mut v[1]);
    val = v[0] + v[1];

    if is_empty_tree(&stree[0][0]) || is_empty_tree(&stree[1][0]) {
        sstree[0].sptr = "";
        sstree[0].len = 0;
    } else {
        let mut left_tree = stree[0][0].clone();
        let mut right_tree = stree[1][0].clone();
        remove_paren(&mut left_tree);
        remove_paren(&mut right_tree);
        add_tree(&left_tree, &right_tree, &mut sstree[0]);
    }

    if is_empty_tree(&stree[0][1]) || is_empty_tree(&stree[1][1]) {
        sstree[1].sptr = "";
        sstree[1].len = 0;
    } else {
        let mut left_tree = stree[0][1].clone();
        let mut right_tree = stree[1][1].clone();
        remove_paren(&mut left_tree);
        remove_paren(&mut right_tree);
        add_tree(&left_tree, &right_tree, &mut sstree[1]);
    }

    let output = format!(
        "({})[{}]({})",
        &sstree[0].sptr[..sstree[0].len],
        val,
        &sstree[1].sptr[..sstree[1].len]
    );

    out.sptr = &output;
    out.len = output.len();
}

fn main() {
    let mut input_tree = [String::new(), String::new()];
    let mut tree = [MyStr { sptr: "", len: 0 }; 3];

    for i in 0..2 {
        io::stdin().read_line(&mut input_tree[i]).unwrap();
        let trimmed = input_tree[i].trim();
        tree[i].sptr = trimmed;
        tree[i].len = trimmed.len();
    }

    let mut result_tree = MyStr { sptr: "", len: 0 };
    add_tree(&tree[0], &tree[1], &mut result_tree);
    display(&result_tree);
}
