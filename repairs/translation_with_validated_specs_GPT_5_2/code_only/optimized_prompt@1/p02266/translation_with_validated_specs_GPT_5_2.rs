use std::io::{self, Read};

#[derive(Clone)]
struct StackNode {
    data: i32,
    next: Option<Box<StackNode>>,
}

#[derive(Clone)]
struct PoolNode {
    area: i32,
    x: i32,
    next: Option<Box<PoolNode>>,
}

// Prints areas from tail to head, each preceded by a space, matching the C recursion.
fn output(p: &Option<Box<PoolNode>>, out: &mut String) {
    if let Some(node) = p.as_ref() {
        output(&node.next, out);
        out.push_str(&format!(" {}", node.area));
    }
}

fn main() {
    // Read entire stdin as bytes; C code uses scanf("%c") which reads every byte including '\n'.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let mut x: i32 = 1;
    let mut stack: Option<Box<StackNode>> = None;
    let mut pool: Option<Box<PoolNode>> = None;

    for &b in &input {
        let ch = b as char;
        match ch {
            '\\' => {
                let new_node = Box::new(StackNode { data: x, next: stack });
                stack = Some(new_node);
            }
            '_' => {
                // do nothing
            }
            '/' => {
                if stack.is_none() {
                    // do nothing
                } else {
                    // Pop stack top, but we need its data first.
                    let top_data = stack.as_ref().unwrap().data;

                    if pool.is_none() {
                        pool = Some(Box::new(PoolNode {
                            area: x - top_data,
                            x: top_data,
                            next: None,
                        }));
                    } else {
                        // We will potentially merge pools while top_data < pool.x
                        // This mirrors the C logic that repeatedly removes the head pool node
                        // and adds its area into the next node, until the condition fails.
                        if top_data < pool.as_ref().unwrap().x {
                            // While there is a next node and next.x > top_data:
                            loop {
                                let cond = {
                                    let head = pool.as_ref().unwrap();
                                    match head.next.as_ref() {
                                        Some(next) => next.x > top_data,
                                        None => false,
                                    }
                                };
                                if !cond {
                                    break;
                                }

                                // Remove head, add its area to new head (old head.next).
                                let mut old_head = pool.take().unwrap();
                                let old_area = old_head.area;
                                let mut new_head = old_head.next.take().unwrap();
                                new_head.area += old_area;
                                pool = Some(new_head);
                            }

                            // Now add the new area to current head and set its x to top_data.
                            {
                                let head = pool.as_mut().unwrap();
                                head.area += x - top_data;
                                head.x = top_data;
                            }
                        } else {
                            // Push new pool node to front.
                            let new_pool = Box::new(PoolNode {
                                area: x - top_data,
                                x: top_data,
                                next: pool,
                            });
                            pool = Some(new_pool);
                        }
                    }

                    // Pop stack node (free in C).
                    let mut old = stack.take().unwrap();
                    stack = old.next.take();
                }
            }
            _ => {
                // Any other character: C switch has no default, so do nothing.
            }
        }
        x += 1;
    }

    // Sum and count pools.
    let mut sum: i32 = 0;
    let mut count: i32 = 0;
    {
        let mut p = pool.as_ref();
        while let Some(node) = p {
            sum += node.area;
            count += 1;
            p = node.next.as_ref();
        }
    }

    // Match C output formatting exactly.
    let mut out = String::new();
    out.push_str(&format!("{}\n{}", sum, count));
    output(&pool, &mut out);
    out.push('\n');

    print!("{}", out);
}
