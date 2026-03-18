use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Pool {
    area: i32,
    x: i32,
}

fn main() {
    // Read entire stdin as bytes; C code uses scanf("%c") which reads every byte including '\n'.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    // STACK is only used as a stack of x positions for '\'.
    let mut stack: Vec<i32> = Vec::new();

    // POOL is maintained as a singly-linked list with head at `pool` in C.
    // We'll represent it as a Vec with index 0 being the head (front).
    let mut pool: Vec<Pool> = Vec::new();

    let mut x: i32 = 1;

    for &b in &input {
        let ch = b as char;
        match ch {
            '\\' => {
                stack.push(x);
            }
            '_' => {
                // do nothing
            }
            '/' => {
                if let Some(sx) = stack.pop() {
                    if pool.is_empty() {
                        pool.push(Pool {
                            area: x - sx,
                            x: sx,
                        });
                    } else {
                        // In C, `pool` is the head node.
                        if sx < pool[0].x {
                            // This block merges nested pools.
                            // The C code's pointer manipulation is a bit convoluted, but its effect is:
                            // while next exists and next.x > sx:
                            //   next.area += head.area
                            //   delete head
                            // then:
                            //   head.area += (x - sx)
                            //   head.x = sx
                            while pool.len() >= 2 && pool[1].x > sx {
                                let head_area = pool[0].area;
                                pool[1].area = pool[1].area.wrapping_add(head_area);
                                pool.remove(0); // free(head)
                            }
                            pool[0].area = pool[0].area.wrapping_add(x - sx);
                            pool[0].x = sx;
                        } else {
                            // Push new pool at head.
                            pool.insert(
                                0,
                                Pool {
                                    area: x - sx,
                                    x: sx,
                                },
                            );
                        }
                    }
                }
            }
            _ => {
                // Any other character is ignored, but still increments x (as in C switch default: do nothing).
            }
        }
        x = x.wrapping_add(1); // preserve C int wraparound behavior as closely as possible
    }

    // Sum and count over the pool list.
    let mut sum: i32 = 0;
    let mut count: i32 = 0;
    for p in &pool {
        sum = sum.wrapping_add(p.area);
        count = count.wrapping_add(1);
    }

    // Output format must match C exactly:
    // printf("%d\n%d",sum,count); output(pool); printf("\n");
    print!("{}\n{}", sum, count);

    // output() prints areas from tail to head, each preceded by a space.
    for p in pool.iter().rev() {
        print!(" {}", p.area);
    }
    print!("\n");
}
