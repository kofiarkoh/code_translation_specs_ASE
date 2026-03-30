#include <stdio.h>
#include <stdlib.h>

#define M 998244353
#define MAX_SIZE 1000 // Assuming a maximum size for arrays d and e

int d[MAX_SIZE] = {0}; // Array to store updates based on input
int e[MAX_SIZE] = {0}; // Array to store updates based on input

int main() {
    int a, s = 0, r = 1; // Initialize cumulative sum s and result r
    int count;

    // Read the first integer which indicates the number of subsequent integers
    if (scanf("%d", &count) != 1) {
        return 0; // Terminate if the first input is not a valid integer
    }

    for (int i = 0; i < count; i++) {
        if (scanf("%d", &a) != 1) {
            break; // Terminate if a non-integer input is encountered
        }

        // Update arrays d and e based on the current input a
        for (int t = 0; t <= s; t++) {
            if (t + a < MAX_SIZE) { // Ensure we do not exceed array bounds
                d[t + a] = (d[t + a] + (d[t] * 2) % M) % M; // Update d
                e[t + a] = (e[t + a] + e[t]) % M; // Update e
            }
        }

        s += a; // Increment cumulative sum
        r = (r * 3) % M; // Update result r
    }

    // Final result computation
    for (int t = s / 2; t >= 0; t--) {
        r = (r - (d[t] * 3) % M + M) % M; // Adjust r based on d
    }
    r = (r + e[s / 2]) % M; // Adjust r based on e

    // Output the final result
    printf("%d\n", r);
    return 0;
}
