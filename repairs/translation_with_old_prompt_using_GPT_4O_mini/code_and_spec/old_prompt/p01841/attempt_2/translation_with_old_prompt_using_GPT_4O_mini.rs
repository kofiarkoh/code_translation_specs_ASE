use std::io;

struct MyStr<'a> {
    sptr: &'a str,
    len: usize,
}

fn display(str: &MyStr) {
    print!("{}", str.sptr);
    println!(); // Ensures a newline after the output
}

fn analyze_tree(in_tree: &MyStr, left: &mut MyStr, right: &mut MyStr, val: &mut i32) {
    let mut lv = 0;
    let mut left_end = 0;
    let mut right_start = 0;

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

    left.sptr = &in_tree.sptr[..left_end];
    left.len = left_end;

    right.sptr = &in_tree.sptr[right_start..];
    right.len = in_tree.len - right_start;
}

fn is_empty_tree(in_tree: &MyStr) -> bool {
    in_tree.len == 2
}

fn remove_paren(in_tree: &mut MyStr) {
    in_tree.len -= 2;
    in_tree.sptr = &in_tree.sptr[1..in_tree.len + 1]; // Adjust pointer to skip parentheses
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
        let mut left_tree = MyStr {
            sptr: stree[0][0].sptr,
            len: stree[0][0].len,
        };
        let mut right_tree = MyStr {
            sptr: stree[1][0].sptr,
            len: stree[1][0].len,
        };
        remove_paren(&mut left_tree);
        remove_paren(&mut right_tree);
        add_tree(&left_tree, &right_tree, &mut sstree[0]);
    }

    if is_empty_tree(&stree[0][1]) || is_empty_tree(&stree[1][1]) {
        sstree[1].sptr = "";
        sstree[1].len = 0;
    } else {
        let mut left_tree = MyStr {
            sptr: stree[0][1].sptr,
            len: stree[0][1].len,
        };
        let mut right_tree = MyStr {
            sptr: stree[1][1].sptr,
            len: stree[1][1].len,
        };
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
    let mut input_tree = vec![String::new(), String::new()];
    io::stdin().read_line(&mut input_tree[0]).unwrap();
    io::stdin().read_line(&mut input_tree[1]).unwrap();

    let tree = [
        MyStr {
            sptr: &input_tree[0].trim(),
            len: input_tree[0].trim().len(),
        },
        MyStr {
            sptr: &input_tree[1].trim(),
            len: input_tree[1].trim().len(),
        },
        MyStr { sptr: "", len: 0 },
    ];

    let mut result_tree = MyStr { sptr: "", len: 0 };
    add_tree(&tree[0], &tree[1], &mut result_tree);
    display(&result_tree);
}
