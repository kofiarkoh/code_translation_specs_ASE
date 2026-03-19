use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Store directed edges (u->v) in arrays a,b of length 2*(n-1)
    let m = 2usize.saturating_mul(n.saturating_sub(1));
    let mut a: Vec<i32> = vec![0; m + 1]; // +1 for sentinel slot used by C code
    let mut b: Vec<i32> = vec![0; m + 1];

    for i in 0..(n.saturating_sub(1)) {
        let u_in: i32 = it.next().unwrap().parse().unwrap();
        let v_in: i32 = it.next().unwrap().parse().unwrap();
        let u = u_in - 1;
        let v = v_in - 1;

        let e0 = 2 * i;
        a[e0] = u;
        b[e0] = v;
        a[e0 + 1] = v;
        b[e0 + 1] = u;
    }

    // The C code heap-sorts edge indices by key a[idx] (ascending).
    // We reproduce that by pushing (a[idx], idx) into a min-heap.
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    for idx in 0..m {
        heap.push(Reverse((a[idx], idx)));
    }

    // c is the sorted list of edge indices by a[edge]
    let mut c: Vec<usize> = vec![0; m + 1];
    for i in 0..m {
        c[i] = heap.pop().unwrap().0 .1;
    }

    // Sentinel as in C:
    // c[m] = m; a[m] = -1; (so a[c[m]] == -1, never equals any vertex id)
    c[m] = m;
    a[m] = -1;

    // BFS routine matching the C logic:
    // - depth initialized to -1
    // - start depth[start]=1
    // - uses a stack-like array q with l as size (LIFO), not FIFO.
    //   For a tree, this still visits all vertices and assigns depths along the unique path.
    fn traverse(n: usize, start: usize, a: &[i32], b: &[i32], c: &[usize], m: usize) -> Vec<i32> {
        let mut depth = vec![-1i32; n];
        let mut q = vec![0usize; n];
        let mut l: usize = 0;

        depth[start] = 1;
        q[0] = start;
        l = 1;

        while l > 0 {
            l -= 1;
            let v = q[l];

            // Binary search for first position with a[c[pos]] >= v
            let mut min: isize = -1;
            let mut max: isize = m as isize; // note: m is last real edge index; c[m] is sentinel
            while max - min > 1 {
                let mid = (max + min) / 2;
                if a[c[mid as usize]] < v as i32 {
                    min = mid;
                } else {
                    max = mid;
                }
            }

            // Iterate all edges with a == v
            let mut pos = max as usize;
            while a[c[pos]] == v as i32 {
                let to = b[c[pos]] as usize;
                if depth[to] < 0 {
                    depth[to] = depth[v] + 1;
                    q[l] = to;
                    l += 1;
                }
                pos += 1;
            }
        }

        depth
    }

    // First traversal from vertex 0 to find farthest vertex s by depth
    let depth0 = traverse(n, 0, &a, &b, &c, m);
    let mut s: usize = 0;
    for i in 0..n {
        if depth0[s] < depth0[i] {
            s = i;
        }
    }

    // Second traversal from s; maximum depth equals diameter length in vertices (per spec)
    let depth1 = traverse(n, s, &a, &b, &c, m);
    let mut d: i32 = 0;
    for i in 0..n {
        if d < depth1[i] {
            d = depth1[i];
        }
    }

    // SPEC: output "Second" iff D mod 3 == 0, where D is diameter length in vertices.
    // Here d is that diameter length in vertices (depth starts at 1).
    if d % 3 == 0 {
        print!("Second\n");
    } else {
        print!("First\n");
    }

    // Note: The original C prints based on (max + 1) % 3, where max is the maximum depth.
    // That would correspond to (D + 1) % 3, which conflicts with the specification.
    // We follow the specification as the source of truth.
}
