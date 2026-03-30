#include <stdio.h>

void process(int N, int *A, int *B) {
    // Ensure N is within the specified range [0, 11]
    N = N % 12; // Modulo operation as per specification

    // Iterate N times to update A and B
    for (int i = 0; i < N; i++) {
        // Update A and B based on their previous values
        int tempA = *A; // Store the current value of A
        int tempB = *B; // Store the current value of B
        
        // Example arithmetic operations (assumed based on typical patterns)
        *A = tempA + tempB; // Update A to be the sum of A and B
        *B = tempA - tempB; // Update B to be the difference of A and B
    }
}

int main() {
    int N; // Input for number of iterations
    int A, B; // Input integers

    // Example input (in a real scenario, these would be read from user or other sources)
    N = 5; // Example value for N
    A = 3; // Example initial value for A
    B = 2; // Example initial value for B

    // Call the processing function
    process(N, &A, &B);

    // Output the final values of A and B
    printf("Final A: %d, Final B: %d\n", A, B);
    return 0;
}
