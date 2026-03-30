#include <stdio.h>

long long compute_total_count(int N, long long *sequence) {
    long long total_count = 0;

    for (int i = 0; i < N; i++) {
        // Add half of the current integer to the total count
        total_count += sequence[i] / 2;

        // Check if the current integer is odd and if the next integer exists and is greater than one
        if (i < N - 1 && (sequence[i] % 2 != 0) && (sequence[i + 1] > 1)) {
            sequence[i + 1]--; // Decrement the next integer
            total_count++; // Increment the total count
        }
    }

    // The last integer is processed by adding half of its value to the total count
    // This is already handled in the loop, so no additional action is needed here.

    return total_count;
}

int main() {
    int N;
    scanf("%d", &N); // Read the number of elements

    long long sequence[100000]; // Array to hold the sequence

    for (int i = 0; i < N; i++) {
        scanf("%lld", &sequence[i]); // Read each long long integer
    }

    long long result = compute_total_count(N, sequence);
    printf("%lld\n", result); // Output the total count

    return 0;
}
