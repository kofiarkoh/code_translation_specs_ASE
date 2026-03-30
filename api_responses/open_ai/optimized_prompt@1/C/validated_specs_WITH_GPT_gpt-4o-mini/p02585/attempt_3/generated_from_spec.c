#include <stdio.h>
#include <limits.h>

long long max_cumulative_value(int N, int K, int P[], int C[]) {
    long long max_value = LLONG_MIN; // Initialize to minimum long long value
    long long cumulative[N]; // Array to store cumulative values
    int visited[N]; // Array to track visited indices
    int steps; // Step counter

    // Initialize cumulative values and visited array
    for (int i = 0; i < N; i++) {
        cumulative[i] = C[i]; // Start with initial cumulative values
        visited[i] = 0; // Mark all indices as unvisited
    }

    // Iterate through each index to compute maximum cumulative value
    for (int start = 0; start < N; start++) {
        steps = 0; // Reset step counter for each starting index
        int current = start; // Start from the current index
        long long current_cumulative = cumulative[start]; // Initialize current cumulative value

        // Reset visited array for this iteration
        for (int i = 0; i < N; i++) {
            visited[i] = 0; // Mark all indices as unvisited
        }

        // Traverse the sequence defined by P
        while (steps < K) {
            if (visited[current]) {
                // Cycle detected, break out of the loop
                break;
            }
            visited[current] = 1; // Mark current index as visited
            current_cumulative += C[current]; // Update cumulative value
            max_value = (current_cumulative > max_value) ? current_cumulative : max_value; // Update max value
            current = P[current] - 1; // Move to the next index (adjust for 0-based index)
            steps++; // Increment step counter
        }
    }

    return max_value; // Return the maximum cumulative value found
}

int main() {
    // Example usage
    int N = 5;
    int K = 3;
    int P[] = {2, 3, 4, 5, 1}; // Example mapping
    int C[] = {1, 2, 3, 4, 5}; // Example cumulative values

    long long result = max_cumulative_value(N, K, P, C);
    printf("Maximum cumulative value: %lld\n", result); // Output the result

    return 0;
}
