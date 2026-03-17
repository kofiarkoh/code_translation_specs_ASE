use std::io::{self, BufRead};

#[derive(Debug)]
struct Stack {
    data: usize,
    next: Option<Box<Stack>>,
}

#[derive(Debug)]
struct Pool {
    area: usize,
    x: usize,
    next: Option<Box<Pool>>,
}

fn output(pool: &Option<Box<Pool>>) {
    if let Some(p) = pool {
        output(&p.next);
        print!(" {}", p.area);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut stack: Option<Box<Stack>> = None;
    let mut pool: Option<Box<Pool>> = None;
    let mut x = 1;

    for line in reader.lines() {
        let line = line.unwrap();
        for in_char in line.chars() {
            match in_char {
                '\\' => {
                    let new_stack = Box::new(Stack {
                        data: x,
                        next: stack.take(),
                    });
                    stack = Some(new_stack);
                }
                '_' => {}
                '/' => {
                    if let Some(stack_node) = stack.take() {
                        let stack_data = stack_node.data;

                        if pool.is_none() {
                            pool = Some(Box::new(Pool {
                                area: x - stack_data,
                                x: stack_data,
                                next: None,
                            }));
                        } else {
                            let mut p = pool.as_mut().unwrap();
                            if stack_data < p.x {
                                while let Some(next_pool) = p.next.as_mut() {
                                    if next_pool.x > stack_data {
                                        next_pool.area += p.area;
                                        p = next_pool;
                                    } else {
                                        break;
                                    }
                                }
                                p.area += x - stack_data;
                                p.x = stack_data;
                            } else {
                                let new_pool = Box::new(Pool {
                                    area: x - stack_data,
                                    x: stack_data,
                                    next: pool,
                                });
                                pool = Some(new_pool);
                            }
                        }
                        stack = stack_node.next; // Move to the next stack element
                    }
                }
                _ => {}
            }
            x += 1;
        }
    }

    let mut sum = 0;
    let mut count = 0;
    let mut p = &pool;

    while let Some(pool_node) = p {
        sum += pool_node.area;
        count += 1;
        p = &pool_node.next;
    }

    println!("{}\n{}", sum, count);
    output(&pool);
    println!(); // Ensure a newline after the output

    // Fix to ensure correct formatting of output
    if let Some(pool_node) = &pool {
        print!("{}", pool_node.area);
        let mut p = &pool_node.next;
        while let Some(next_pool) = p {
            print!(" {}", next_pool.area);
            p = &next_pool.next;
        }
    }
    println!(); // Ensure a newline after the output
}
