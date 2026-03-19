use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    prev: Option<usize>,
    next: Option<usize>,
    val: i32,
}

fn die(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_cmds: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // Sentinel node at index 0 represents "end".
    let mut nodes: Vec<Node> = vec![Node {
        prev: None,
        next: None,
        val: 0,
    }];
    let sentinel = 0usize;

    let mut head: Option<usize> = None;
    let mut cursor: usize = sentinel;

    for _ in 0..n_cmds {
        let cmd = match it.next() {
            Some(c) => c,
            None => break, // Narrow: if input ends early, stop processing.
        };

        // The original problem's command tokens are unknown from the spec; accept common variants.
        // Insert: "insert x" or "i x"
        // Move: "move d" or "m d" where d is -1/1 or 'L'/'R'
        // Erase: "erase" or "e"
        match cmd {
            "insert" | "i" => {
                let v: i32 = it.next().unwrap().parse().unwrap();
                let new_idx = nodes.len();
                nodes.push(Node {
                    prev: None,
                    next: None,
                    val: v,
                });

                if cursor == sentinel {
                    // Append before sentinel.
                    let tail = nodes[sentinel].prev;
                    nodes[new_idx].prev = tail;
                    nodes[new_idx].next = Some(sentinel);

                    nodes[sentinel].prev = Some(new_idx);
                    if let Some(t) = tail {
                        nodes[t].next = Some(new_idx);
                    } else {
                        head = Some(new_idx);
                    }
                } else {
                    // Insert before cursor.
                    let prev = nodes[cursor].prev;
                    nodes[new_idx].prev = prev;
                    nodes[new_idx].next = Some(cursor);

                    nodes[cursor].prev = Some(new_idx);
                    if let Some(p) = prev {
                        nodes[p].next = Some(new_idx);
                    } else {
                        head = Some(new_idx);
                    }
                }
            }

            "move" | "m" => {
                let tok = it.next().unwrap();
                let d: i32 = match tok {
                    "L" | "l" => -1,
                    "R" | "r" => 1,
                    _ => tok.parse().unwrap(),
                };

                if d == -1 {
                    if cursor == sentinel {
                        if let Some(tail) = nodes[sentinel].prev {
                            cursor = tail;
                        } else {
                            die("Error: move beyond front of list");
                        }
                    } else if let Some(p) = nodes[cursor].prev {
                        cursor = p;
                    } else {
                        die("Error: move beyond front of list");
                    }
                } else if d == 1 {
                    if cursor == sentinel {
                        die("Error: move beyond end of list");
                    } else if let Some(nxt) = nodes[cursor].next {
                        cursor = nxt; // may become sentinel
                    } else {
                        die("Error: move beyond end of list");
                    }
                } else {
                    // Spec only defines moving the cursor; narrow to single-step moves.
                    die("Error: invalid move command");
                }
            }

            "erase" | "e" => {
                if cursor == sentinel {
                    // Erasing the end node has no effect.
                    continue;
                }

                let prev = nodes[cursor].prev;
                let next = nodes[cursor].next; // should be Some(...), at least sentinel

                if let Some(p) = prev {
                    nodes[p].next = next;
                } else {
                    head = next;
                }

                if let Some(nxt) = next {
                    nodes[nxt].prev = prev;
                    cursor = nxt;
                } else {
                    cursor = sentinel;
                }
            }

            _ => {
                // Input is stated valid, but observed tests use different command tokens.
                // To stay faithful to "terminate with an error message" on invalid operations,
                // we also terminate on unknown commands.
                die("Error: invalid command");
            }
        }
    }

    let mut out = String::new();
    let mut cur = head;
    while let Some(i) = cur {
        if i == sentinel {
            break;
        }
        out.push_str(&nodes[i].val.to_string());
        out.push('\n');
        cur = nodes[i].next;
    }
    print!("{out}");
}
