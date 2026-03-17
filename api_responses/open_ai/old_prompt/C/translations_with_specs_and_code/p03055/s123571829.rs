use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; 2 * n - 1];
    let mut b = vec![0; 2 * n - 1];
    let mut depth = vec![-1; n];
    let mut edges = vec![(0, 0); 2 * n - 2];
    let mut l = 0;

    for i in 0..(n - 1) {
        let edge = lines.next().unwrap().unwrap();
        let mut parts = edge.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        
        a[l] = u - 1; // Convert to 0-based index
        b[l] = v - 1; // Convert to 0-based index
        edges[i * 2] = (a[l], b[l]);
        edges[i * 2 + 1] = (b[l], a[l]);
        l += 2;
    }

    let mut c = vec![0; 2 * n - 2];
    for i in 0..(2 * n - 2) {
        c[i] = i;
    }

    // First BFS to find the maximum depth
    depth[0] = 1;
    let mut queue = vec![0];
    let mut q_len = 1;

    while q_len > 0 {
        q_len -= 1;
        let i = queue[q_len];
        let mut min = -1;
        let mut max = 2 * n - 2;

        while max - min > 1 {
            let mid = (max + min) / 2;
            if a[c[mid]] < i {
                min = mid;
            } else {
                max = mid;
            }
        }

        while a[c[max]] == i {
            if depth[b[c[max]]] < 0 {
                depth[b[c[max]]] = depth[i] + 1;
                queue.push(b[c[max]]);
                q_len += 1;
            }
            max += 1;
        }
    }

    let mut s = 0;
    for i in 0..n {
        if depth[s] < depth[i] {
            s = i;
        }
    }

    // Reset depth for second BFS
    depth.fill(-1);
    depth[s] = 1;
    queue = vec![s];
    q_len = 1;

    while q_len > 0 {
        q_len -= 1;
        let i = queue[q_len];
        let mut min = -1;
        let mut max = 2 * n - 2;

        while max - min > 1 {
            let mid = (max + min) / 2;
            if a[c[mid]] < i {
                min = mid;
            } else {
                max = mid;
            }
        }

        while a[c[max]] == i {
            if depth[b[c[max]]] < 0 {
                depth[b[c[max]]] = depth[i] + 1;
                queue.push(b[c[max]]);
                q_len += 1;
            }
            max += 1;
        }
    }

    let max_depth = *depth.iter().max().unwrap();
    if (max_depth + 1) % 3 > 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
