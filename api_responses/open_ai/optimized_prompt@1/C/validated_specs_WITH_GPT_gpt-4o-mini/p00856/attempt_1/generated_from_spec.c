#include <stdio.h>
#include <stdlib.h>
#include <float.h>

double compute_probability(int N, int T, int L, int B, int *L_positions, int *B_positions) {
    // Memoization array initialized to maximum double value
    double **memo = (double **)malloc((T + 1) * sizeof(double *));
    for (int i = 0; i <= T; i++) {
        memo[i] = (double *)malloc((N + 1) * sizeof(double));
        for (int j = 0; j <= N; j++) {
            memo[i][j] = DBL_MAX; // Initialize to maximum double value
        }
    }

    // Base case: if no turns left, probability is 0.0 unless all inputs are zero
    if (N == 0 && T == 0 && L == 0 && B == 0) {
        for (int i = 0; i <= T; i++) {
            free(memo[i]);
        }
        free(memo);
        return 0.0;
    }

    // Recursive function to calculate probability
    double probability_helper(int turns_left, int position) {
        // If out of bounds or position is an obstacle or barrier
        if (position < 0 || position >= N) return 0.0;
        for (int i = 0; i < L; i++) {
            if (L_positions[i] == position) return 0.0; // Hit an obstacle
        }
        for (int i = 0; i < B; i++) {
            if (B_positions[i] == position) return 0.0; // Hit a barrier
        }

        // Check memoization
        if (memo[turns_left][position] != DBL_MAX) {
            return memo[turns_left][position];
        }

        // Base case: if no turns left, return 1.0 (success)
        if (turns_left == 0) {
            return 1.0;
        }

        // Recursive case: evaluate possible moves
        double prob = 0.0;
        for (int move = -1; move <= 1; move++) {
            prob += probability_helper(turns_left - 1, position + move);
        }

        // Average the probabilities of possible moves
        prob /= 2.0; // Two possible moves (-1 and +1)

        // Store in memoization array
        memo[turns_left][position] = prob;
        return prob;
    }

    // Start the recursion from position 0
    double result = probability_helper(T, 0);

    // Free the memoization array
    for (int i = 0; i <= T; i++) {
        free(memo[i]);
    }
    free(memo);

    // Ensure the result is between 0.0 and 1.0
    if (result < 0.0) return 0.0;
    if (result > 1.0) return 1.0;
    return result;
}

int main() {
    // Example usage
    int N = 10; // Total positions
    int T = 3;  // Total turns
    int L = 2;  // Number of obstacles
    int B = 1;  // Number of barriers
    int L_positions[] = {2, 5}; // Positions of obstacles
    int B_positions[] = {7};     // Positions of barriers

    double probability = compute_probability(N, T, L, B, L_positions, B_positions);
    printf("Probability of successfully completing the game: %f\n", probability);
    return 0;
}
