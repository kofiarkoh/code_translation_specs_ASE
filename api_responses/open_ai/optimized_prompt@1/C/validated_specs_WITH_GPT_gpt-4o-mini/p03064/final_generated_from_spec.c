#include <stdio.h>
#include <stdlib.h>

#define M 998244353

int main() {
    int d[1000] = {0}; // Array to hold values, size is arbitrary for this example
    int e[1000] = {0}; // Array to hold values, size is arbitrary for this example
    int r = 1; // Result variable initialized to 1
    int s = 0; // Cumulative sum
    int a; // Input integer
    int count; // Number of integers to process

    // Read the first integer which indicates the count of subsequent integers
    if (scanf("%d", &count) != 1) {
        return 0; // Terminate if the first input is not a valid integer
    }

    for (int i = 0; i < count; i++) {
        if (scanf("%d", &a) != 1) {
            break; // Terminate if a non-integer input is encountered
        }
        
        s += a; // Update cumulative sum

        // Update arrays d and e based on the current input a
        for (int t = 0; t <= s; t++) {
            if (t + a < 1000) { // Ensure we do not exceed array bounds
                d[t + a] = (d[t + a] + (d[t] * 2) % M) % M; // Update d
                e[t + a] = (e[t + a] + e[t]) % M; // Update e
            }
        }

        r = (r * 3) % M; // Update result variable r
    }

    // Final result computation
    for (int t = s / 2; t >= 0; t--) {
        r = (r - (d[t] * 3) % M + M) % M; // Adjust r based on d
    }
    r = (r + e[s / 2]) % M; // Adjust r based on e

    printf("%d\n", r); // Output the final result
    return 0;
}
