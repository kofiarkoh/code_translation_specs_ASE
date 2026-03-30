#include <stdio.h>
#include <stdlib.h>
#include <float.h>

double compute_probability(int N, int T, int L, int B, int *obstacles, int *barriers, int remaining_moves, int remaining_turns, double **memo) {
    // Base cases
    if (remaining_moves <= 0) return 1.0; // Success if no moves left
    if (remaining_turns <= 0) return 0.0; // Failure if no turns left

    // Check memoization
    if (memo[remaining_moves][remaining_turns] != DBL_MAX) {
        return memo[remaining_moves][remaining_turns];
    }

    double total_probability = 0.0;
    int valid_moves = 0;

    // Evaluate possible moves (1 to 6)
    for (int move = 1; move <= 6; move++) {
        int next_position = remaining_moves - move; // Calculate next position

        // Check if next position is valid
        if (next_position < 0 || next_position >= N) {
            continue; // Invalid position, skip
        }

        // Check if the next position is an obstacle or barrier
        int is_obstacle = 0, is_barrier = 0;
        for (int i = 0; i < L; i++) {
            if (obstacles[i] == next_position) {
                is_obstacle = 1;
                break;
            }
        }
        for (int i = 0; i < B; i++) {
            if (barriers[i] == next_position) {
                is_barrier = 1;
                break;
            }
        }

        // Calculate probability based on the position type
        if (!is_obstacle && !is_barrier) {
            total_probability += compute_probability(N, T, L, B, obstacles, barriers, next_position, remaining_turns - 1, memo);
            valid_moves++;
        }
    }

    // If there are valid moves, average the probabilities
    if (valid_moves > 0) {
        total_probability /= valid_moves;
    }

    // Store in memoization array
    memo[remaining_moves][remaining_turns] = total_probability;
    return total_probability;
}

double calculate_game_probability(int N, int T, int L, int B, int *obstacles, int *barriers) {
    // Handle the case where all inputs are zero
    if (N == 0 && T == 0 && L == 0 && B == 0) {
        return 0.0; // Graceful termination
    }

    // Initialize memoization array
    double **memo = (double **)malloc((N + 1) * sizeof(double *));
    for (int i = 0; i <= N; i++) {
        memo[i] = (double *)malloc((T + 1) * sizeof(double));
        for (int j = 0; j <= T; j++) {
            memo[i][j] = DBL_MAX; // Initialize to maximum double value
        }
    }

    // Compute the probability starting with all moves and turns
    double probability = compute_probability(N, T, L, B, obstacles, barriers, N, T, memo);

    // Free memoization array
    for (int i = 0; i <= N; i++) {
        free(memo[i]);
    }
    free(memo);

    return probability; // Return the computed probability
}

int main() {
    // Example usage
    int N = 10; // Total positions
    int T = 3;  // Total turns
    int L = 2;  // Number of obstacles
    int B = 1;  // Number of barriers
    int obstacles[] = {2, 5}; // Positions of obstacles
    int barriers[] = {7};      // Positions of barriers

    double probability = calculate_game_probability(N, T, L, B, obstacles, barriers);
    printf("Probability of successfully completing the game: %f\n", probability);
    return 0;
}
