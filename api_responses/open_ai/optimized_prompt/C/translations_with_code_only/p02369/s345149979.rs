use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{null_mut, NonNull};
use std::io::{self, Write};

#[repr(C)]
struct Edge {
    tgt: i32,
    cost: i32,
}

#[repr(C)]
struct EdgeListNode {
    e: Edge,
    nxt: *mut EdgeListNode,
}

#[repr(C)]
struct EdgeList {
    front: *mut EdgeListNode,
    back: *mut EdgeListNode,
    num_edge: u32,
}

#[repr(C)]
struct AdjacencyList {
    src: *mut EdgeList,
    num_vertex: u32,
}

fn make_adjacency_list(v: i32) -> *mut AdjacencyList {
    let layout = Layout::new::<AdjacencyList>();
    let graph = unsafe { alloc(layout) as *mut AdjacencyList };

    unsafe {
        (*graph).src = alloc(Layout::array::<EdgeList>(v as usize).unwrap()) as *mut EdgeList;
        for i in 0..v {
            (*(*graph).src.add(i as usize)).front = null_mut();
            (*(*graph).src.add(i as usize)).back = null_mut();
            (*(*graph).src.add(i as usize)).num_edge = 0;
        }
        (*graph).num_vertex = v as u32;
    }

    graph
}

fn push_edge(graph: *mut AdjacencyList, source: i32, edge: Edge) {
    let layout = Layout::new::<EdgeListNode>();
    let n = unsafe { alloc(layout) as *mut EdgeListNode };
    unsafe {
        (*n).e = edge;
        (*n).nxt = null_mut();

        if (*graph).src.add(source as usize).front.is_null() {
            (*(*graph).src.add(source as usize)).front = n;
        } else {
            (*(*graph).src.add(source as usize)).back.as_mut().unwrap().nxt = n;
        }
        (*(*graph).src.add(source as usize)).back = n;
        (*(*graph).src.add(source as usize)).num_edge += 1;
    }
}

fn dump_adjacency_list(graph: *const AdjacencyList) {
    let v = unsafe { (*graph).num_vertex };
    for i in 0..v {
        let mut n = unsafe { (*(*graph).src.add(i as usize)).front };
        print!("{} --> ", i);
        while !n.is_null() {
            unsafe {
                print!("{:3} ", (*n).e.tgt);
                n = (*n).nxt;
            }
        }
        println!();
    }
}

fn is_cyclic(graph: *mut AdjacencyList) -> i32 {
    let v = unsafe { (*graph).num_vertex };
    let mut in_degree = vec![0; v as usize];

    // Calculate in-degrees
    for i in 0..v {
        let mut n = unsafe { (*(*graph).src.add(i as usize)).front };
        while !n.is_null() {
            in_degree[unsafe { (*n).e.tgt as usize }] += 1;
            n = unsafe { (*n).nxt };
        }
    }

    for _ in 0..v {
        let mut flag = true;
        for i in 0..v {
            if in_degree[i as usize] == 0 {
                in_degree[i as usize] = -1; // Mark as visited
                flag = false;

                let mut n = unsafe { (*(*graph).src.add(i as usize)).front };
                while !n.is_null() {
                    in_degree[unsafe { (*n).e.tgt as usize }] -= 1;
                    n = unsafe { (*n).nxt };
                }
                break;
            }
        }
        if flag {
            return 1; // Cycle detected
        }
    }
    0 // No cycle
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let v: i32 = iter.next().unwrap().parse().unwrap();
    let e: i32 = iter.next().unwrap().parse().unwrap();

    let graph = make_adjacency_list(v);

    for _ in 0..e {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let s: i32 = iter.next().unwrap().parse().unwrap();
        let t: i32 = iter.next().unwrap().parse().unwrap();
        let edge = Edge { tgt: t, cost: 0 };
        push_edge(graph, s, edge);
    }

    let res = is_cyclic(graph);
    println!("{}", res);

    // Clean up memory (not shown here for brevity)
}
