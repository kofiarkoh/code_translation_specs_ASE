use std::io::{self, BufRead, Read};

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
    let mut reader = stdin.lock();
    let mut in_char = [0; 1];
    let mut x = 1;
    let mut stack: Option<Box<Stack>> = None;
    let mut pool: Option<Box<Pool>> = None;

    while reader.read(&mut in_char).unwrap() > 0 {
        let in_char = in_char[0] as u8 as char; // Change to u8 for valid cast
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
                    let tmp_stack = stack.as_ref().unwrap();
                    pool = Some(Box::new(Pool {
                        area: x - tmp_stack.data,
                        x: tmp_stack.data,
                        next: None,
                    }));
                } else {
                    let tmp_stack = stack.as_ref().unwrap();
                    if tmp_stack.data < pool.as_ref().unwrap().x {
                        let mut p = pool.as_mut().unwrap().next.take();
                        while let Some(mut next_pool) = p {
                            if next_pool.x > tmp_stack.data {
                                next_pool.area += pool.as_ref().unwrap().area;
                                p = next_pool.next.take();
                                pool = pool.as_mut().unwrap().next.take();
                                continue;
                            }
                            break;
                        }
                        pool.as_mut().unwrap().area += x - tmp_stack.data;
                        pool.as_mut().unwrap().x = tmp_stack.data;
                    } else {
                        let new_pool = Box::new(Pool {
                            area: x - tmp_stack.data,
                            x: tmp_stack.data,
                            next: pool.take(),
                        });
                        pool = Some(new_pool);
                    }
                }
                stack = stack.and_then(|s| s.next);
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
