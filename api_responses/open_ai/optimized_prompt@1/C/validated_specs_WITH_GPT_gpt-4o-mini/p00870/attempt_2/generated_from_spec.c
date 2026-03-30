#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAX_STRINGS 100
#define MAX_STRING_LENGTH 21 // 20 characters + 1 for null terminator
#define MAX_CONCATENATED_LENGTH 5001 // 5000 characters + 1 for null terminator

int count_distinct_combinations(int n, int m, char strings[MAX_STRINGS][MAX_STRING_LENGTH], char *concatenated) {
    int count = 0;
    int total_combinations = 1 << n; // 2^n combinations
    int matched[total_combinations]; // Array to track matched combinations
    memset(matched, 0, sizeof(matched)); // Initialize to 0

    // Iterate through the concatenated string
    for (int i = 0; i < strlen(concatenated); i++) {
        for (int j = 0; j < n; j++) {
            // Check if the substring matches the j-th string
            if (strncmp(&concatenated[i], strings[j], strlen(strings[j])) == 0) {
                // Create a bitmask for the matched combination
                int bitmask = 1 << j;
                // Update the matched array for this position
                for (int k = 0; k < total_combinations; k++) {
                    if (matched[k]) {
                        matched[k | bitmask] = 1; // Mark the new combination as matched
                    }
                }
                matched[bitmask] = 1; // Mark the current string as matched
            }
        }
        // Check if we have matched all strings (i.e., the full combination)
        if (matched[total_combinations - 1]) {
            count++;
        }
    }

    return count;
}

int main() {
    int n, m;
    char strings[MAX_STRINGS][MAX_STRING_LENGTH];
    char concatenated[MAX_CONCATENATED_LENGTH];

    while (1) {
        // Read n and m
        scanf("%d %d", &n, &m);
        if (n == 0 && m == 0) break; // Termination condition

        // Read n strings
        for (int i = 0; i < n; i++) {
            scanf("%s", strings[i]);
        }

        // Read the concatenated string
        scanf("%s", concatenated);

        // Get the count of distinct combinations
        int result = count_distinct_combinations(n, m, strings, concatenated);
        printf("%d\n", result);
    }

    return 0;
}
