#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>

#define MAX_POINTS 128 // Based on the assumption that N <= 127

// Function to calculate the cost of movement between two points
int calculate_cost(int x1, int y1, int x2, int y2, int A, int B, int C, int D) {
    // Calculate the direct distance
    int direct_distance = (int) sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
    
    // Adjust cost based on boundary constraints
    int adjusted_cost = direct_distance;
    if (x1 < A || x1 > B || y1 < C || y1 > D || x2 < A || x2 > B || y2 < C || y2 > D) {
        adjusted_cost += 10; // Penalty for moving out of bounds
    }
    
    return adjusted_cost;
}

// Function to find the minimum cost of movement across points
int find_minimum_cost(int N, int A, int B, int C, int D, int X[], int Y[]) {
    int min_cost = INT_MAX;

    // Iterate through all pairs of points to calculate costs
    for (int i = 0; i < N; i++) {
        for (int j = i + 1; j <= N; j++) {
            int cost = calculate_cost(X[i], Y[i], X[j], Y[j], A, B, C, D);
            if (cost < min_cost) {
                min_cost = cost; // Update minimum cost if a lower cost is found
            }
        }
    }
    
    return min_cost;
}

int main() {
    int N, A, B, C, D;
    int X[MAX_POINTS], Y[MAX_POINTS];

    // Read input for N, A, B, C, D
    if (scanf("%d %d %d %d %d", &N, &A, &B, &C, &D) != 5 || N <= 0) {
        return -1; // Error code for invalid input
    }

    // Read coordinates
    for (int i = 0; i <= N; i++) {
        if (scanf("%d %d", &X[i], &Y[i]) != 2) {
            return -1; // Error code for invalid coordinate input
        }
    }

    // Calculate the minimum cost
    int result = find_minimum_cost(N, A, B, C, D, X, Y);
    
    // Output the result
    printf("%d\n", result);
    return 0;
}
