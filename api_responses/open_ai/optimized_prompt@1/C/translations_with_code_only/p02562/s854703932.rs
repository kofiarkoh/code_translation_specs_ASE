use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[derive(Clone, Copy)]
struct Data {
    key: i64,
    id: i32,
}

struct MinHeap {
    obj: *mut Data,
    size: usize,
}

impl MinHeap {
    fn new(capacity: usize) -> Self {
        let layout = Layout::array::<Data>(capacity).unwrap();
        let obj = unsafe { alloc(layout) as *mut Data };
        MinHeap { obj, size: 0 }
    }

    fn push(&mut self, x: Data) {
        self.size += 1;
        let mut i = self.size;
        let mut j = i >> 1;
        unsafe {
            *self.obj.add(i) = x;
        }
        while j > 0 {
            if unsafe { (*self.obj.add(i)).key < (*self.obj.add(j)).key } {
                unsafe {
                    let tmp = *self.obj.add(j);
                    *self.obj.add(j) = *self.obj.add(i);
                    *self.obj.add(i) = tmp;
                }
                i = j;
                j >>= 1;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Data {
        let mut i = 1;
        let mut j = 2;
        let output = unsafe { *self.obj.add(1) };
        unsafe {
            *self.obj.add(1) = *self.obj.add(self.size);
            self.size -= 1;
        }
        while j <= self.size {
            if j < self.size && unsafe { (*self.obj.add(j ^ 1)).key < (*self.obj.add(j)).key } {
                j ^= 1;
            }
            if unsafe { (*self.obj.add(j)).key < (*self.obj.add(i)).key } {
                unsafe {
                    let tmp = *self.obj.add(j);
                    *self.obj.add(j) = *self.obj.add(i);
                    *self.obj.add(i) = tmp;
                }
                i = j;
                j <<= 1;
            } else {
                break;
            }
        }
        output
    }
}

struct Edge {
    id: i32,
    head: i32,
    tail: i32,
    flow: i32,
    cap: i32,
    cost: i32,
}

struct List {
    next: *mut List,
    e: *mut Edge,
}

const SUP: i64 = 1 << 60;
const INF: i64 = -SUP;

fn succesive_shortest_path(N: usize, M: usize, adj: &mut *mut List, e: &mut [Edge], b: &mut [i32]) -> i64 {
    let mut flag = vec![0; N + 1];
    let mut def = vec![0; N + 1];
    let mut ans = 0;
    let mut dist = vec![0; N + 1];
    let mut pi = vec![SUP; N + 1];
    let mut prev: Vec<*mut Edge> = vec![ptr::null_mut(); N + 1];
    let mut h_def = MinHeap::new(N + 1);
    let mut h_dist = MinHeap::new(M * 2 + 1);

    for i in 0..M {
        ans += (e[i].cost as i64) * (e[i].flow as i64);
    }

    for i in 1..=N {
        pi[i] = SUP;
    }

    for i in 1..=N {
        let mut p = unsafe { *adj.add(i) };
        def[i] = b[i];
        while !p.is_null() {
            unsafe {
                def[i] += (*(*p).e).flow - e[(*(*p).e).id as usize ^ 1].flow;
            }
            p = unsafe { (*p).next };
        }
        if def[i] < 0 {
            let d = Data { key: def[i] as i64, id: i as i32 };
            h_def.push(d);
            if pi[i] == SUP {
                pi[i] = 0;
                for j in 1..=N {
                    let mut k = 0;
                    flag[0] = 0;
                    while k < M {
                        if e[k].flow == e[k].cap {
                            k += 1;
                            continue;
                        }
                        let u = e[k].tail;
                        let w = e[k].head;
                        if pi[u] != SUP && pi[w] > pi[u] + e[k].cost as i64 {
                            flag[0] = 1;
                            pi[w] = pi[u] + e[k].cost as i64;
                        }
                        k += 1;
                    }
                    if flag[0] == 0 {
                        break;
                    }
                }
                if j > N {
                    return INF;
                }
            }
        }
    }

    let mut s;
    let mut t;
    let mut min;
    while h_def.size > 0 {
        let d = h_def.pop();
        s = d.id;
        for u in 1..=N {
            flag[u as usize] = 0;
            dist[u as usize] = SUP;
            prev[u as usize] = ptr::null_mut();
        }
        dist[s as usize] = 0;
        h_dist.size = 0;
        let d = Data { key: 0, id: s };
        h_dist.push(d);

        while h_dist.size > 0 {
            let d = h_dist.pop();
            let u = d.id;
            if flag[u as usize] != 0 {
                continue;
            } else {
                flag[u as usize] = 1;
            }

            let mut p = unsafe { *adj.add(u as usize) };
            while !p.is_null() {
                let w = unsafe { (*(*p).e).head };
                if unsafe { (*(*p).e).flow == (*(*p).e).cap } {
                    p = unsafe { (*p).next };
                    continue;
                } else if dist[w as usize] > dist[u as usize] + unsafe { (*(*p).e).cost as i64 } + pi[u as usize] - pi[w as usize] {
                    dist[w as usize] = dist[u as usize] + unsafe { (*(*p).e).cost as i64 } + pi[u as usize] - pi[w as usize];
                    prev[w as usize] = (*p).e;
                    let d = Data { key: dist[w as usize], id: w };
                    h_dist.push(d);
                }
                p = unsafe { (*p).next };
            }
        }
        for t in 1..=N {
            if dist[t as usize] != SUP && def[t as usize] > 0 {
                break;
            }
        }
        if t > N {
            return SUP;
        }

        let mut u = t;
        min = if -def[s as usize] < def[t as usize] {
            -def[s as usize]
        } else {
            def[t as usize]
        };
        while !prev[u as usize].is_null() {
            if min > unsafe { (*prev[u as usize]).cap - (*prev[u as usize]).flow } {
                min = unsafe { (*prev[u as usize]).cap - (*prev[u as usize]).flow };
            }
            u = unsafe { (*prev[u as usize]).tail };
        }
        u = t;
        while !prev[u as usize].is_null() {
            let i = unsafe { (*prev[u as usize]).id };
            if e[i as usize ^ 1].flow > 0 {
                e[i as usize ^ 1].flow -= min;
                e[i as usize].cap -= min;
            } else {
                e[i as usize].flow += min;
                e[i as usize ^ 1].cap += min;
            }
            ans += (e[i as usize].cost as i64) * (min as i64);
            u = unsafe { (*prev[u as usize]).tail };
        }
        for i in 1..=N {
            if dist[i as usize] != SUP {
                pi[i as usize] += dist[i as usize];
            }
        }

        def[s as usize] += min;
        def[t as usize] -= min;
        if def[s as usize] < 0 {
            let d = Data { key: def[s as usize] as i64, id: s };
            h_def.push(d);
        }
    }

    ans
}

fn main() {
    let mut N = 0;
    let mut K = 0;
    let mut A = vec![vec![0; 52]; 52];
    std::io::stdin().read_line(&mut format!("{} {}", N, K)).unwrap();

    for i in 1..=N {
        for j in 1..=N {
            std::io::stdin().read_line(&mut format!("{}", A[i][j])).unwrap();
        }
    }

    let mut adj: Vec<*mut List> = vec![ptr::null_mut(); 103];
    let mut d = vec![List { next: ptr::null_mut(), e: ptr::null_mut() }; 6000];
    let mut e = vec![Edge { id: 0, head: 0, tail: 0, flow: 0, cap: 0, cost: 0 }; 6000];
    let mut k = 0;

    for u in 1..=N {
        for w in N + 1..=N * 2 {
            e[k * 2].id = k * 2;
            e[k * 2].tail = u;
            e[k * 2].head = w;
            e[k * 2].cap = 1;
            e[k * 2].cost = -A[u][w - N];
            e[k * 2 + 1].id = k * 2 + 1;
            e[k * 2 + 1].tail = w;
            e[k * 2 + 1].head = u;
            e[k * 2 + 1].cap = 0;
            e[k * 2 + 1].cost = A[u][w - N];
            d[k * 2].e = &mut e[k * 2];
            d[k * 2 + 1].e = &mut e[k * 2 + 1];
            d[k * 2].next = adj[u as usize];
            d[k * 2 + 1].next = adj[w as usize];
            adj[u as usize] = &mut d[k * 2];
            adj[w as usize] = &mut d[k * 2 + 1];
            k += 1;
        }
    }

    for w in 1..=N {
        let u = N * 2 + 1;
        e[k * 2].id = k * 2;
        e[k * 2].tail = u;
        e[k * 2].head = w;
        e[k * 2].cap = K;
        e[k * 2].cost = 0;
        e[k * 2 + 1].id = k * 2 + 1;
        e[k * 2 + 1].tail = w;
        e[k * 2 + 1].head = u;
        e[k * 2 + 1].cap = 0;
        e[k * 2 + 1].cost = 0;
        d[k * 2].e = &mut e[k * 2];
        d[k * 2 + 1].e = &mut e[k * 2 + 1];
        d[k * 2].next = adj[u as usize];
        d[k * 2 + 1].next = adj[w as usize];
        adj[u as usize] = &mut d[k * 2];
        adj[w as usize] = &mut d[k * 2 + 1];
        k += 1;
    }

    for u in N + 1..=N * 2 {
        let w = N * 2 + 2;
        e[k * 2].id = k * 2;
        e[k * 2].tail = u;
        e[k * 2].head = w;
        e[k * 2].cap = K;
        e[k * 2].cost = 0;
        e[k * 2 + 1].id = k * 2 + 1;
        e[k * 2 + 1].tail = w;
        e[k * 2 + 1].head = u;
        e[k * 2 + 1].cap = 0;
        e[k * 2 + 1].cost = 0;
        d[k * 2].e = &mut e[k * 2];
        d[k * 2 + 1].e = &mut e[k * 2 + 1];
        d[k * 2].next = adj[u as usize];
        d[k * 2 + 1].next = adj[w as usize];
        adj[u as usize] = &mut d[k * 2];
        adj[w as usize] = &mut d[k * 2 + 1];
        k += 1;
    }

    let mut b = vec![0; 103];
    let mut argmax = 0;
    let mut max = 0;

    for i in 1..=N * K {
        b[N * 2 + 1] = -i;
        b[N * 2 + 2] = i;
        let tmp = -succesive_shortest_path(N * 2 + 2, k * 2, &mut adj.as_mut_ptr(), &mut e, &mut b);
        if tmp != SUP && max < tmp {
            max = tmp;
            argmax = i;
        }
    }
    println!("{}", max);

    let mut ans = vec![vec!['.'; 52]; 52];
    for i in 1..=N {
        for j in 1..=N {
            ans[i][j] = '.';
        }
    }
    b[N * 2 + 1] = -argmax;
    b[N * 2 + 2] = argmax;
    for j in 0..k {
        e[j * 2].flow = 0;
        e[j * 2 + 1].cap = 0;
    }
    succesive_shortest_path(N * 2 + 2, k * 2, &mut adj.as_mut_ptr(), &mut e, &mut b);
    for j in 0..k {
        let u = e[j * 2].tail;
        let w = e[j * 2].head;
        if u <= N && w <= N * 2 && e[j * 2].flow == 1 {
            ans[u][w - N] = 'X';
        }
    }
    for i in 1..=N {
        println!("{}", ans[i].iter().collect::<String>());
    }
}
