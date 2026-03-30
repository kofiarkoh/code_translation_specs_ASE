#include <stdio.h>
#include <limits.h>

#define MAX_N 400

// Global variables
int N; // Number of elements in the sequence
int sequence[MAX_N]; // Input sequence of integers
long long memo[MAX_N][MAX_N]; // Memoization table for storing minimum costs
long long cumulative_sum[MAX_N]; // Cumulative sum for efficient cost calculation

// Function to compute the cumulative sums
void compute_cumulative_sum() {
    cumulative_sum[0] = sequence[0];
    for (int i = 1; i < N; i++) {
        cumulative_sum[i] = cumulative_sum[i - 1] + sequence[i];
    }
}

// Recursive function to compute the minimum cost of partitioning
long long min_cost(int l, int r) {
    // If the result is already computed, return it
    if (memo[l][r] != -1) {
        return memo[l][r];
    }

    // Base case: if l == r, the cost is just the single element
    if (l == r) {
        return sequence[l];
    }

    long long cost = LLONG_MAX; // Initialize cost to maximum possible value

    // Try partitioning the sequence at every possible point
    for (int mid = l; mid < r; mid++) {
        long long left_cost = min_cost(l, mid);
        long long right_cost = min_cost(mid + 1, r);
        long long total_cost = left_cost + right_cost + (cumulative_sum[r] - (l > 0 ? cumulative_sum[l - 1] : 0));
        
        // Update the minimum cost found
        if (total_cost < cost) {
            cost = total_cost;
        }
    }

    // Store the computed cost in the memoization table
    memo[l][r] = cost;
    return cost;
}

// Main function to read input and compute the minimum cost
int main() {
    // Read the number of elements
    scanf("%d", &N);
    
    // Read the sequence of integers
    for (int i = 0; i < N; i++) {
        scanf("%d", &sequence[i]);
    }

    // Initialize the memoization table with -1
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            memo[i][j] = -1;
        }
    }

    // Compute cumulative sums for efficient cost calculation
    compute_cumulative_sum();

    // Compute the minimum cost for the entire sequence
    long long result = min_cost(0, N - 1);

    // Output the result
    printf("%lld\n", result);
    return 0;
}
