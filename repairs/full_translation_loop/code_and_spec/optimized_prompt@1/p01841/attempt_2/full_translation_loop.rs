use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct MyStr<'a> {
    sptr: &'a str,
    len: usize,
}

fn display(str_: &MyStr) {
    print!("{}", str_.sptr);
    // Uncomment for debug information
    // println!("{{{}}}", str_.len);
    println!();
}

fn analyze_tree(in_tree: &MyStr, left: &mut MyStr, right: &mut MyStr, val: &mut i32) {
    let mut lv = 0;
    let mut left_len = 0;
    let mut right_start = 0;

    for (i, c) in in_tree.sptr.chars().enumerate() {
        match c {
            '[' => {
                if lv == 0 {
                    left_len = i;
                    *val = in_tree.sptr[i + 1..].parse::<i32>().unwrap();
                }
                lv += 1;
            }
            ']' => {
                if lv == 1 {
                    right_start = i + 1;
                }
                lv -= 1;
            }
            '(' => lv += 1,
            ')' => lv -= 1,
            _ => {}
        }
    }

    left.sptr = &in_tree.sptr[..left_len];
    left.len = left_len;

    right.sptr = &in_tree.sptr[right_start..];
    right.len = in_tree.len - right_start;
}

fn is_empty_tree(in_tree: &MyStr) -> bool {
    in_tree.len == 2
}

fn remove_paren(in_tree: &mut MyStr) {
    in_tree.len -= 2;
    in_tree.sptr = &in_tree.sptr[1..];
}

fn add_tree(in1: &MyStr, in2: &MyStr, out: &mut MyStr) {
    let mut stree = [[MyStr { sptr: "", len: 0 }; 2]; 2];
    let mut sstree = [MyStr { sptr: "", len: 0 }; 2];
    let mut v = [0; 2];
    let mut val = 0;

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
    let stdin = io::stdin();
    let input = stdin.lock().lines().take(2).collect::<Vec<_>>();
    
    let tree1 = input[0].as_ref().unwrap();
    let tree2 = input[1].as_ref().unwrap();

    let my_tree1 = MyStr {
        sptr: tree1,
        len: tree1.len(),
    };
    let my_tree2 = MyStr {
        sptr: tree2,
        len: tree2.len(),
    };
    let mut result_tree = MyStr {
        sptr: "",
        len: 0,
    };

    add_tree(&my_tree1, &my_tree2, &mut result_tree);
    display(&result_tree);
}
