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

    // Arrays sized as in C (globals), but allocated locally.
    // Use i32 to match C int behavior (including potential negative values like -1).
    let mut a = vec![0i32; 400_005];
    let mut b = vec![0i32; 400_005];
    let mut h = vec![0i32; 400_005]; // heap stores edge indices
    let mut l: i32 = 0;

    // Read edges; C loop mutates i inside, but net effect is i runs 0..n-2
    for i in 0..(n_usize.saturating_sub(1)) {
        let idx = 2 * i;
        let mut ai: i32 = it.next().unwrap().parse().unwrap();
        let mut bi: i32 = it.next().unwrap().parse().unwrap();
        ai -= 1;
        bi -= 1;

        a[idx] = ai;
        b[idx] = bi;
        a[idx + 1] = bi;
        b[idx + 1] = ai;
    }

    // Heap helpers: comp_h compares by a[h[*]]; returns 1 or -1 (never 0), matching C.
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

    // push(ne)
    let mut push = |ne: i32,
                    a: &Vec<i32>,
                    h: &mut Vec<i32>,
                    l: &mut i32,
                    comp_h: &mut dyn FnMut(i32, i32, &Vec<i32>, &Vec<i32>) -> i32,
                    swap_h: &mut dyn FnMut(i32, i32, &mut Vec<i32>)| {
        let p0 = *l;
        h[p0 as usize] = ne;
        *l += 1;
        let mut p = p0;
        while p > 0 {
            let parent = (p - 1) / 2;
            if comp_h(parent, p, a, h) > 0 {
                swap_h(parent, p, h);
            }
            p = parent;
        }
    };

    // pop() -> i32 (returns edge index)
    let mut pop = |a: &Vec<i32>,
                   h: &mut Vec<i32>,
                   l: &mut i32,
                   comp_h: &mut dyn FnMut(i32, i32, &Vec<i32>, &Vec<i32>) -> i32,
                   swap_h: &mut dyn FnMut(i32, i32, &mut Vec<i32>)| -> i32 {
        *l -= 1;
        swap_h(0, *l, h);
        let mut p: i32 = 0;
        loop {
            let left = 2 * p + 1;
            let right = 2 * p + 2;
            if right < *l {
                if comp_h(left, right, a, h) > 0 {
                    if comp_h(p, right, a, h) > 0 {
                        swap_h(p, right, h);
                    }
                    p = right;
                } else {
                    if comp_h(p, left, a, h) > 0 {
                        swap_h(p, left, h);
                    }
                    p = left;
                }
            } else if left < *l {
                if comp_h(p, left, a, h) > 0 {
                    swap_h(p, left, h);
                }
                p = left;
            } else {
                break;
            }
        }
        h[*l as usize]
    };

    // Build heap with edges 0..2n-3
    l = 0;
    let m_edges = 2 * n - 2; // i32
    for i in 0..(m_edges as i32) {
        push(i, &a, &mut h, &mut l, &mut comp_h, &mut swap_h);
    }

    // c is sorted list of edge indices by a[edge] ascending (via heap)
    let mut c = vec![0i32; 400_005];
    for i in 0..(m_edges as usize) {
        c[i] = pop(&a, &mut h, &mut l, &mut comp_h, &mut swap_h);
    }
    c[m_edges as usize] = m_edges;
    a[m_edges as usize] = -1; // sentinel

    // BFS-like traversal using binary search over sorted edges by a[edge]
    let mut depth = vec![-1i32; 200_005];
    let mut q = vec![0i32; 200_005];

    // First BFS from node 0
    depth[0] = 1;
    q[0] = 0;
    l = 1;
    while l > 0 {
        l -= 1;
        let i = q[l as usize];

        let mut minv: i32 = -1;
        let mut maxv: i32 = m_edges;
        while maxv - minv > 1 {
            let mid = (maxv + minv) / 2;
            let am = a[c[mid as usize] as usize];
            if am < i {
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
    let mut s: usize = 0;
    for i in 0..n_usize {
        if depth[s] < depth[i] {
            s = i;
        }
    }

    // Second BFS from s
    for i in 0..n_usize {
        depth[i] = -1;
    }
    depth[s] = 1;
    q[0] = s as i32;
    l = 1;
    while l > 0 {
        l -= 1;
        let i = q[l as usize];

        let mut minv: i32 = -1;
        let mut maxv: i32 = m_edges;
        while maxv - minv > 1 {
            let mid = (maxv + minv) / 2;
            let am = a[c[mid as usize] as usize];
            if am < i {
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
    for i in 0..n_usize {
        if maxd < depth[i] {
            maxd = depth[i];
        }
    }

    if (maxd + 1) % 3 > 0 {
        print!("First\n");
    } else {
        print!("Second\n");
    }
}
