#include <stdio.h>

int main() {
    long long T[2]; // Time factors
    long long A[2]; // Parameters A
    long long B[2]; // Parameters B
    long long result; // Result variable

    // Input reading (assumed to be valid long long integers)
    scanf("%lld %lld", &T[0], &T[1]);
    scanf("%lld %lld", &A[0], &A[1]);
    scanf("%lld %lld", &B[0], &B[1]);

    // Preconditions check: T[1] and T[2] must not be zero
    if (T[0] == 0 || T[1] == 0) {
        return 1; // Exit if precondition is violated
    }

    // Check if the linear combination of A and B is equal
    if (A[0] * T[1] == B[0] * T[0]) {
        printf("infinity\n"); // Output "infinity" if equal
    } else {
        // If A[1] > B[1], modify A and B
        if (A[0] > B[0]) {
            A[0] -= B[0]; // Modify A[1] based on B[1]
            B[0] -= A[0]; // Modify B[1] based on A[1]
        }
        // Calculate the result based on differences
        result = (A[0] - B[0]) + (A[1] - B[1]);
        printf("%lld\n", result); // Output the calculated result
    }

    return 0;
}
