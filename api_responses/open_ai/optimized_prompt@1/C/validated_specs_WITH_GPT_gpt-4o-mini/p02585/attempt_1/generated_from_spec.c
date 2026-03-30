#include <stdio.h>

long long max_cumulative_value(int N, int K, int P[], int C[]) {
    long long max_value = 0; // To track the maximum cumulative value
    long long cumulative[N];  // Array to store cumulative values

    // Initialize cumulative values to 0
    for (int i = 0; i < N; i++) {
        cumulative[i] = 0;
    }

    // Iterate through the elements up to K steps
    for (int step = 0; step < K; step++) {
        for (int i = 0; i < N; i++) {
            // Update cumulative value based on the relationship defined by P
            cumulative[i] += C[i]; // Add current cumulative value
            if (P[i] >= 0 && P[i] < N) { // Ensure P[i] is within bounds
                cumulative[P[i]] += cumulative[i]; // Update based on P
            }
            // Update max_value if the current cumulative value is greater
            if (cumulative[i] > max_value) {
                max_value = cumulative[i];
            }
        }
    }

    return max_value; // Return the maximum cumulative value found
}
