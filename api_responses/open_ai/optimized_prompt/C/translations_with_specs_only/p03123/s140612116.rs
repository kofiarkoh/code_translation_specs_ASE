fn max_allocation(r: u64, b: u64) -> u64 {
    // The maximum feasible allocation k is bounded by 2,222,222.
    const MAX_K: u64 = 2_222_222;

    // We will use binary search to find the maximum k.
    let mut low = 0;
    let mut high = MAX_K;

    while low < high {
        let mid = (low + high + 1) / 2; // Use upper mid to avoid infinite loop

        // Check if mid is a feasible allocation
        // The condition for feasibility is that we can allocate mid resources
        // and still have enough resources left for the remaining allocations.
        if mid <= r && mid <= b && (r + b) >= 2 * mid {
            low = mid; // mid is feasible, search in the upper half
        } else {
            high = mid - 1; // mid is not feasible, search in the lower half
        }
    }

    // Return the maximum feasible allocation k minus one
    low // low will be the maximum feasible k
}
