#include <stdio.h>

int count_valid_subsets(int N, int A, int *set) {
    // Calculate the total sum of the set
    int total_sum = 0;
    for (int i = 0; i < N; i++) {
        total_sum += set[i];
    }

    // Determine the maximum k based on the total sum and A
    int max_k = total_sum / A < N ? total_sum / A : N;

    // Dynamic programming table to count subsets
    // dp[i][j] will hold the count of subsets using first i elements that sum to j
    int dp[N + 1][total_sum + 1];
    
    // Initialize the dp table
    for (int i = 0; i <= N; i++) {
        for (int j = 0; j <= total_sum; j++) {
            dp[i][j] = 0; // No subsets initially
        }
    }
    dp[0][0] = 1; // One way to select an empty subset with sum 0

    // Populate the dp table
    for (int i = 1; i <= N; i++) {
        for (int j = 0; j <= total_sum; j++) {
            dp[i][j] = dp[i - 1][j]; // Exclude the current element
            if (j >= set[i - 1]) {
                dp[i][j] += dp[i - 1][j - set[i - 1]]; // Include the current element
            }
        }
    }

    // Count valid subsets whose sum equals k * A for k = 1 to max_k
    int count = 0;
    for (int k = 1; k <= max_k; k++) {
        count += dp[N][k * A]; // Add the count of subsets that sum to k * A
    }

    return count; // Return the total count of valid subsets
}

int main() {
    int N, A;
    // Example input reading (not specified in the task, but necessary for testing)
    scanf("%d %d", &N, &A);
    int set[N];
    for (int i = 0; i < N; i++) {
        scanf("%d", &set[i]);
    }

    // Call the function and print the result
    int result = count_valid_subsets(N, A, set);
    printf("%d\n", result);

    return 0;
}
