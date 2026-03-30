#include <stdio.h>

long long compute_score(const char *sequence, int n, int k) {
    long long score = 0;
    int countD = 0; // Count of 'D's in the current window

    // Initialize the first window
    for (int i = 0; i < k && i < n; i++) {
        if (sequence[i] == 'D') {
            countD++;
        } else if (sequence[i] == 'M') {
            score += countD; // Each 'M' contributes based on prior 'D's
        }
    }

    // Slide the window across the sequence
    for (int i = k; i < n; i++) {
        // Remove the character going out of the window
        if (sequence[i - k] == 'D') {
            countD--;
        } else if (sequence[i - k] == 'M') {
            // No action needed for 'M' going out
        }

        // Add the new character coming into the window
        if (sequence[i] == 'D') {
            countD++;
        } else if (sequence[i] == 'M') {
            score += countD; // Each 'M' contributes based on prior 'D's
        }
    }

    return score;
}

void process_queries(int n, const char *sequence, int q, const int *k) {
    for (int j = 0; j < q; j++) {
        long long result = compute_score(sequence, n, k[j]);
        printf("%lld\n", result); // Output the score for each query
    }
}

int main() {
    int n; // Length of the character sequence
    scanf("%d", &n); // Read the length
    char sequence[n + 1]; // +1 for null terminator
    scanf("%s", sequence); // Read the character sequence

    int q; // Number of queries
    scanf("%d", &q); // Read the number of queries
    int k[q]; // Array to hold window sizes
    for (int j = 0; j < q; j++) {
        scanf("%d", &k[j]); // Read each window size
    }

    process_queries(n, sequence, q, k); // Process and output results for each query

    return 0;
}
