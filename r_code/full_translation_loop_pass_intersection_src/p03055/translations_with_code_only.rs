use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let n_usize = n as usize;

    // Arrays sized exactly as in C (globals), initialized to 0.
    let mut a: Vec<i32> = vec![0; 400_005];
    let mut b: Vec<i32> = vec![0; 400_005];
    let mut h: Vec<i32> = vec![0; 400_005];
    let mut l: i32;

    // Read edges; preserve the odd C loop structure exactly.
    // for (i = 0; i < n - 1; i++) { i *= 2; ...; i /= 2; }
    let mut i: i32 = 0;
    while i < n - 1 {
        i *= 2;
        let ai: i32 = it.next().unwrap().parse().unwrap();
        let bi: i32 = it.next().unwrap().parse().unwrap();
        let idx = i as usize;

        a[idx] = ai;
        b[idx] = bi;
        a[idx] -= 1;
        b[idx] -= 1;

        a[idx + 1] = b[idx];
        b[idx + 1] = a[idx];

        i /= 2;
        i += 1;
    }

    // Heap helpers (min-heap by a[h[*]]; comp returns 1 or -1, never 0, matching C)
    let mut comp_h = |x: i32, y: i32, a: &Vec<i32>, h: &Vec<i32>| -> i32 {
        let hx = h[x as usize] as usize;
        let hy = h[y as usize] as usize;
        if a[hx] > a[hy] { 1 } else { -1 }
    };
    let mut swap_h = |x: i32, y: i32, h: &mut Vec<i32>| {
        let xi = x as usize;
        let yi = y as usize;
        let f = h[xi];
        h[xi] = h[yi];
        h[yi] = f;
    };

    let mut push = |ne: i32,
                    a: &Vec<i32>,
                    h: &mut Vec<i32>,
                    l_ref: &mut i32,
                    comp_h: &mut dyn FnMut(i32, i32, &Vec<i32>, &Vec<i32>) -> i32,
                    swap_h: &mut dyn FnMut(i32, i32, &mut Vec<i32>)| {
        let l0 = *l_ref;
        h[l0 as usize] = ne;
        let mut p = l0;
        *l_ref += 1;
        while p > 0 {
            let parent = (p - 1) / 2;
            if comp_h(parent, p, a, h) > 0 {
                swap_h(parent, p, h);
            }
            p = parent;
        }
    };

    let mut pop = |a: &Vec<i32>,
                   h: &mut Vec<i32>,
                   l_ref: &mut i32,
                   comp_h: &mut dyn FnMut(i32, i32, &Vec<i32>, &Vec<i32>) -> i32,
                   swap_h: &mut dyn FnMut(i32, i32, &mut Vec<i32>)| -> i32 {
        *l_ref -= 1;
        let l_now = *l_ref;
        swap_h(0, l_now, h);
        let mut p: i32 = 0;
        loop {
            let lcur = *l_ref;
            if 2 * p + 2 < lcur {
                if comp_h(2 * p + 1, 2 * p + 2, a, h) > 0 {
                    if comp_h(p, 2 * p + 2, a, h) > 0 {
                        swap_h(p, 2 * p + 2, h);
                    }
                    p = 2 * p + 2;
                } else {
                    if comp_h(p, 2 * p + 1, a, h) > 0 {
                        swap_h(p, 2 * p + 1, h);
                    }
                    p = 2 * p + 1;
                }
            } else if 2 * p + 1 < lcur {
                if comp_h(p, 2 * p + 1, a, h) > 0 {
                    swap_h(p, 2 * p + 1, h);
                }
                p = 2 * p + 1;
            } else {
                break;
            }
        }
        h[*l_ref as usize]
    };

    // Build heap of indices 0..2n-3
    l = 0;
    let m = 2 * n - 2; // number of directed edges
    for idx in 0..m {
        push(
            idx,
            &a,
            &mut h,
            &mut l,
            &mut comp_h,
            &mut swap_h,
        );
    }

    // Pop into c (sorted by a[edge] ascending, but with C's comparator behavior)
    let mut c: Vec<i32> = vec![0; 400_005];
    for idx in 0..m {
        c[idx as usize] = pop(&a, &mut h, &mut l, &mut comp_h, &mut swap_h);
    }
    c[m as usize] = m;
    a[m as usize] = -1; // sentinel

    // BFS-like traversal using binary search over c by a[c[*]]
    let mut depth: Vec<i32> = vec![-1; 200_005];
    let mut q: Vec<i32> = vec![0; 200_005];

    depth[0] = 1;
    q[0] = 0;
    l = 1;

    while l > 0 {
        l -= 1;
        i = q[l as usize];

        let mut minv: i32 = -1;
        let mut maxv: i32 = m;
        while maxv - minv > 1 {
            let mid = (maxv + minv) / 2;
            if a[c[mid as usize] as usize] < i {
                minv = mid;
            } else {
                maxv = mid;
            }
        }
        while a[c[maxv as usize] as usize] == i {
            let to = b[c[maxv as usize] as usize] as usize;
            if depth[to] < 0 {
                depth[to] = depth[i as usize] + 1;
                q[l as usize] = to as i32;
                l += 1;
            }
            maxv += 1;
        }
    }

    // Find farthest node s
    let mut s: i32 = 0;
    for v in 0..n_usize {
        if depth[s as usize] < depth[v] {
            s = v as i32;
        }
    }

    // Second BFS from s
    for v in 0..n_usize {
        depth[v] = -1;
    }
    depth[s as usize] = 1;
    q[0] = s;
    l = 1;

    while l > 0 {
        l -= 1;
        i = q[l as usize];

        let mut minv: i32 = -1;
        let mut maxv: i32 = m;
        while maxv - minv > 1 {
            let mid = (maxv + minv) / 2;
            if a[c[mid as usize] as usize] < i {
                minv = mid;
            } else {
                maxv = mid;
            }
        }
        while a[c[maxv as usize] as usize] == i {
            let to = b[c[maxv as usize] as usize] as usize;
            if depth[to] < 0 {
                depth[to] = depth[i as usize] + 1;
                q[l as usize] = to as i32;
                l += 1;
            }
            maxv += 1;
        }
    }

    // Compute max depth
    let mut maxd: i32 = 0;
    for v in 0..n_usize {
        if maxd < depth[v] {
            maxd = depth[v];
        }
    }

    if (maxd + 1) % 3 > 0 {
        print!("First\n");
    } else {
        print!("Second\n");
    }
}
