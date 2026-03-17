use std::io::{self, BufRead};

#[derive(Debug)]
struct MyStr<'a> {
    sptr: &'a str,
    len: usize,
}

fn display(str_: &MyStr) {
    print!("{}", str_.sptr);
    println!(); // Ensures a newline is printed after the string
}

fn analyze_tree(in_tree: &MyStr, left: &mut MyStr, right: &mut MyStr, val: &mut i32) {
    let mut lv = 0;
    let mut left_end = 0;
    let mut right_start = 0;

    left.sptr = in_tree.sptr;

    for (i, c) in in_tree.sptr.chars().enumerate() {
        match c {
            '[' => {
                if lv == 0 {
                    left_end = i;
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

    left.len = left_end;
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
    let input_lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    let tree1 = MyStr {
        sptr: &input_lines[0],
        len: input_lines[0].len(),
    };
    let tree2 = MyStr {
        sptr: &input_lines[1],
        len: input_lines[1].len(),
    };

    let mut result_tree = MyStr {
        sptr: "",
        len: 0,
    };

    add_tree(&tree1, &tree2, &mut result_tree);
    display(&result_tree);
}
