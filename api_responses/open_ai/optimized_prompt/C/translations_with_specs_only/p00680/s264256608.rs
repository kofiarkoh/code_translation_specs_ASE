use std::fmt;

#[derive(Debug)]
struct Edge {
    capacity: f64,
    cost: f64,
}

#[derive(Debug)]
struct Graph {
    nodes: usize,
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(nodes: usize) -> Self {
        Graph {
            nodes,
            edges: vec![vec![]; nodes],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: f64, cost: f64) {
        self.edges[from].push(Edge { capacity, cost });
    }
}

fn gauss_jordan(matrix: &mut Vec<Vec<f64>>, constants: &mut Vec<f64>) -> Result<(), &'static str> {
    let n = matrix.len();
    for i in 0..n {
        // Find pivot
        let pivot = matrix[i][i];
        if pivot == 0.0 {
            return Err("Singular matrix");
        }
        for j in 0..n {
            if j != i {
                let ratio = matrix[j][i] / pivot;
                for k in 0..n {
                    matrix[j][k] -= ratio * matrix[i][k];
                }
                constants[j] -= ratio * constants[i];
            }
        }
    }
    Ok(())
}

fn min_cost_flow(graph: &Graph, source: usize, sink: usize, flow: f64) -> Result<f64, &'static str> {
    // Placeholder for min-cost flow algorithm
    // This should implement a modified Dijkstra's approach
    // For now, we return an error to indicate "impossible"
    Err("impossible")
}

fn process_test_case(
    nodes: usize,
    source: usize,
    sink: usize,
    flow: f64,
    matrix: Vec<Vec<f64>>,
    constants: Vec<f64>,
    edges: Vec<(usize, usize, f64, f64)>,
) -> String {
    if nodes <= 0 || source >= nodes || sink >= nodes || flow < 0.0 {
        return "impossible".to_string();
    }

    let mut graph = Graph::new(nodes);
    for (from, to, capacity, cost) in edges {
        graph.add_edge(from, to, capacity, cost);
    }

    let mut matrix_clone = matrix.clone();
    let mut constants_clone = constants.clone();

    if gauss_jordan(&mut matrix_clone, &mut constants_clone).is_err() {
        return "impossible".to_string();
    }

    match min_cost_flow(&graph, source, sink, flow) {
        Ok(cost) => format!("{:.10}", cost),
        Err(_) => "impossible".to_string(),
    }
}

fn main() {
    // Example usage
    let test_cases = 1; // Number of test cases
    let nodes = 3; // Number of nodes
    let source = 0; // Source node
    let sink = 2; // Sink node
    let flow = 5.0; // Flow to be sent
    let matrix = vec![vec![1.0, 2.0, 3.0], vec![0.0, 1.0, 4.0], vec![0.0, 0.0, 1.0]]; // Coefficients
    let constants = vec![10.0, 20.0, 30.0]; // Constants
    let edges = vec![(0, 1, 10.0, 1.0), (1, 2, 10.0, 1.0)]; // Edges with capacities and costs

    let result = process_test_case(nodes, source, sink, flow, matrix, constants, edges);
    println!("{}", result);
}
