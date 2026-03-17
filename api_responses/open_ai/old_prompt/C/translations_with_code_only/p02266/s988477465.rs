use std::io::{self, BufRead};

struct Stack {
    data: i32,
    next: Option<Box<Stack>>,
}

struct Pool {
    area: i32,
    x: i32,
    next: Option<Box<Pool>>,
}

fn output(p: &Option<Box<Pool>>) {
    if let Some(pool) = p {
        output(&pool.next);
        print!(" {}", pool.area);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    let mut x = 1;
    let mut stack: Option<Box<Stack>> = None;
    let mut pool: Option<Box<Pool>> = None;

    while let Some(Ok(in_char)) = input.next() {
        let in_char = in_char.chars().next().unwrap(); // Get the first character

        match in_char {
            '\\' => {
                let tmp = Box::new(Stack {
                    data: x,
                    next: stack.take(),
                });
                stack = Some(tmp);
            }
            '_' => {}
            '/' => {
                if stack.is_none() {
                    continue;
                }

                if pool.is_none() {
                    let tmp = Box::new(Pool {
                        area: x - stack.as_ref().unwrap().data,
                        x: stack.as_ref().unwrap().data,
                        next: None,
                    });
                    pool = Some(tmp);
                } else {
                    let stack_data = stack.as_ref().unwrap().data;
                    if stack_data < pool.as_ref().unwrap().x {
                        let mut p = pool.as_mut().unwrap().next.take();
                        while let Some(mut next_pool) = p {
                            if next_pool.x > stack_data {
                                next_pool.area += pool.as_ref().unwrap().area;
                                p = next_pool.next.take();
                                pool = pool.as_mut().unwrap().next.take();
                            } else {
                                break;
                            }
                        }
                        pool.as_mut().unwrap().area += x - stack_data;
                        pool.as_mut().unwrap().x = stack_data;
                    } else {
                        let tmp = Box::new(Pool {
                            area: x - stack_data,
                            x: stack_data,
                            next: pool.take(),
                        });
                        pool = Some(tmp);
                    }
                }
                stack = stack.and_then(|s| s.next); // Pop the stack
            }
            _ => {}
        }
        x += 1;
    }

    let mut sum = 0;
    let mut count = 0;
    let mut p = pool.as_ref();

    while let Some(pool_ref) = p {
        sum += pool_ref.area;
        count += 1;
        p = pool_ref.next.as_ref();
    }

    println!("{}\n{}", sum, count);
    output(&pool);
    println!();
}
