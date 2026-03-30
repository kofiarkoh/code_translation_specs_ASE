#include <stdio.h>
#include <limits.h>

long process_pairs(int N, long pairs[][2]) {
    long cumulative_sum = 0; // Cumulative sum of regular integers
    long min_long = LONG_MAX; // Minimum long integer encountered
    long max_value = LONG_MIN; // Maximum computed value

    for (int i = 0; i < N; i++) {
        long long_int = pairs[i][0]; // Long integer from the pair
        int reg_int = pairs[i][1]; // Regular integer from the pair

        // Update cumulative sum of regular integers
        cumulative_sum += reg_int;

        // Update minimum long integer encountered
        if (long_int < min_long) {
            min_long = long_int;
        }

        // Calculate the current value based on cumulative sum and minimum long integer
        long current_value = cumulative_sum - min_long;

        // Update maximum value if the current value is greater
        if (current_value > max_value) {
            max_value = current_value;
        }
    }

    return max_value; // Return the maximum computed value
}

int main() {
    int N; // Number of pairs
    scanf("%d", &N); // Read the number of pairs

    long pairs[N][2]; // Array to hold pairs of integers

    // Read the pairs of integers
    for (int i = 0; i < N; i++) {
        scanf("%ld %d", &pairs[i][0], &pairs[i][1]);
    }

    // Process the pairs and get the maximum value
    long result = process_pairs(N, pairs);

    // Output the result
    printf("%ld\n", result);

    return 0;
}
