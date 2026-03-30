struct BinaryIndexedTree {
    size: usize,
    tree: Vec<usize>,
}

impl BinaryIndexedTree {
    fn new(size: usize) -> Self {
        BinaryIndexedTree {
            size,
            tree: vec![0; size + 1],
        }
    }

    fn update(&mut self, index: usize, value: usize) {
        let mut idx = index;
        while idx <= self.size {
            self.tree[idx] += value;
            idx += idx & !idx + 1; // Move to the next index
        }
    }

    fn query(&self, index: usize) -> usize {
        let mut sum = 0;
        let mut idx = index;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & !idx + 1; // Move to the parent index
        }
        sum
    }
}

fn count(m: usize, a: &[usize], n: usize) -> usize {
    let mut count = 0;
    for &value in a.iter() {
        if value <= m {
            count += 1;
        }
    }
    count
}

fn find_l(n: usize, a: &[usize]) -> usize {
    let mut bit = BinaryIndexedTree::new(2 * n);
    
    // Populate the binary indexed tree
    for &value in a.iter() {
        bit.update(value, 1);
    }

    let mut l = 1;
    let mut r = 2 * n;
    let mut result = 0;

    while l <= r {
        let m = (l + r) / 2;
        let sum = count(m, a, n);

        if 4 * sum >= n * (n + 1) {
            result = m; // Update result if condition is met
            l = m + 1; // Search in the upper half
        } else {
            r = m - 1; // Search in the lower half
        }
    }

    result
}
