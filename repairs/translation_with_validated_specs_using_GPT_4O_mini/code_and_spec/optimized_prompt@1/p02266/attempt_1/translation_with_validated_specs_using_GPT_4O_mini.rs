use std::io::{self, BufRead};

#[derive(Debug)]
struct StackNode {
    data: usize,
    next: Option<Box<StackNode>>,
}

#[derive(Debug)]
struct PoolNode {
    area: usize,
    x: usize,
    next: Option<Box<PoolNode>>,
}

fn output(pool: &Option<Box<PoolNode>>) {
    if let Some(p) = pool {
        output(&p.next);
        print!(" {}", p.area);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut stack: Option<Box<StackNode>> = None;
    let mut pool: Option<Box<PoolNode>> = None;
    let mut x = 1;

    for line in reader.lines() {
        let input = line.unwrap();
        for in_char in input.chars() {
            match in_char {
                '\\' => {
                    let new_node = Box::new(StackNode {
                        data: x,
                        next: stack,
                    });
                    stack = Some(new_node);
                }
                '_' => {}
                '/' => {
                    if let Some(stack_node) = stack.take() {
                        let stack_data = stack_node.data;

                        if pool.is_none() {
                            let new_pool = Box::new(PoolNode {
                                area: x - stack_data,
                                x: stack_data,
                                next: None,
                            });
                            pool = Some(new_pool);
                        } else {
                            let mut current_pool = pool.as_mut().unwrap();
                            if stack_data < current_pool.x {
                                while let Some(next_pool) = current_pool.next.as_mut() {
                                    if next_pool.x > stack_data {
                                        next_pool.area += current_pool.area;
                                        current_pool.next = next_pool.next.take();
                                    } else {
                                        break;
                                    }
                                }
                                current_pool.area += x - stack_data;
                                current_pool.x = stack_data;
                            } else {
                                let new_pool = Box::new(PoolNode {
                                    area: x - stack_data,
                                    x: stack_data,
                                    next: Some(pool.take().unwrap()),
                                });
                                pool = Some(new_pool);
                            }
                        }
                        stack = stack_node.next; // Move to the next stack node
                    }
                }
                _ => {}
            }
            x += 1;
        }
    }

    let mut sum = 0;
    let mut count = 0;
    let mut current_pool = pool.as_ref();

    while let Some(p) = current_pool {
        sum += p.area;
        count += 1;
        current_pool = p.next.as_ref();
    }

    println!("{}\n{}", sum, count);
    output(&pool);
    println!();
}
