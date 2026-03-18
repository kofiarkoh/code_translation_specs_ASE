use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Node {
    label: String,
    children: Vec<Node>,
}

fn is_label_char(c: char) -> bool {
    // Narrow interpretation: labels are any non-whitespace characters except structural tokens.
    !c.is_whitespace() && c != '(' && c != ')' && c != ','
}

fn skip_ws(s: &str, i: &mut usize) {
    while *i < s.len() {
        let ch = s[*i..].chars().next().unwrap();
        if ch.is_whitespace() {
            *i += ch.len_utf8();
        } else {
            break;
        }
    }
}

fn parse_label(s: &str, i: &mut usize) -> String {
    skip_ws(s, i);
    let start = *i;
    while *i < s.len() {
        let ch = s[*i..].chars().next().unwrap();
        if is_label_char(ch) {
            *i += ch.len_utf8();
        } else {
            break;
        }
    }
    s[start..*i].to_string()
}

fn parse_node(s: &str, i: &mut usize) -> Option<Node> {
    skip_ws(s, i);
    if *i >= s.len() {
        return None;
    }

    // Empty input is treated as null tree.
    // Also treat an empty label as null (narrowest reasonable interpretation).
    let label = parse_label(s, i);
    if label.is_empty() {
        return None;
    }

    skip_ws(s, i);
    let mut children = Vec::new();

    // Format supported: label or label(child1,child2,...)
    if *i < s.len() {
        let ch = s[*i..].chars().next().unwrap();
        if ch == '(' {
            *i += 1; // consume '('
            loop {
                skip_ws(s, i);
                if *i >= s.len() {
                    break; // precondition says valid; keep narrow behavior
                }
                let ch2 = s[*i..].chars().next().unwrap();
                if ch2 == ')' {
                    *i += 1; // consume ')'
                    break;
                }

                if let Some(child) = parse_node(s, i) {
                    children.push(child);
                } else {
                    // If a child is "empty", omit it (null subtree).
                }

                skip_ws(s, i);
                if *i >= s.len() {
                    break;
                }
                let sep = s[*i..].chars().next().unwrap();
                if sep == ',' {
                    *i += 1; // consume ','
                    continue;
                } else if sep == ')' {
                    *i += 1; // consume ')'
                    break;
                } else {
                    // Unexpected token; precondition says valid, so stop narrowly.
                    break;
                }
            }
        }
    }

    Some(Node { label, children })
}

fn parse_tree(s: &str) -> Option<Node> {
    let mut i = 0usize;
    let node = parse_node(s, &mut i);
    // Ignore trailing whitespace; precondition says valid.
    node
}

fn merge_trees(a: Option<Node>, b: Option<Node>) -> Option<Node> {
    match (a, b) {
        (None, None) => None,
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (Some(mut x), Some(y)) => {
            // Narrow, format-preserving merge:
            // - Combine root "values" by concatenating labels with '+'.
            // - Combine structures by appending children from the second tree.
            x.label = format!("{}+{}", x.label, y.label);
            x.children.extend(y.children);
            Some(x)
        }
    }
}

fn write_tree(n: &Node, out: &mut String) {
    out.push_str(&n.label);
    if !n.children.is_empty() {
        out.push('(');
        for (idx, ch) in n.children.iter().enumerate() {
            if idx != 0 {
                out.push(',');
            }
            write_tree(ch, out);
        }
        out.push(')');
    }
}

fn main() {
    // Read entire stdin; treat missing lines as empty trees.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let s1 = lines.next().unwrap_or("").trim_end_matches(&['\r', '\n'][..]).to_string();
    let s2 = lines.next().unwrap_or("").trim_end_matches(&['\r', '\n'][..]).to_string();

    let t1 = if s1.trim().is_empty() { None } else { parse_tree(&s1) };
    let t2 = if s2.trim().is_empty() { None } else { parse_tree(&s2) };

    let merged = merge_trees(t1, t2);

    let mut out = String::new();
    if let Some(root) = merged {
        write_tree(&root, &mut out);
    } else {
        // Null tree prints as empty output (narrowest reasonable interpretation).
    }

    print!("{out}");
}
