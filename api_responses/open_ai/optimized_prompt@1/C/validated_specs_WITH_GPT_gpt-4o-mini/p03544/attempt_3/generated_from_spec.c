#include <stdio.h>

long long compute_nth_term(int n) {
    // Define the array to hold the sequence values
    long long l[100];

    // Initialize the first two terms of the sequence
    l[0] = 2; // First term
    l[1] = 1; // Second term

    // Compute the sequence iteratively for n >= 2
    for (int i = 2; i <= n; i++) {
        l[i] = l[i - 2] + l[i - 1]; // Each term is the sum of the two preceding terms
    }

    // Return the nth term
    return l[n];
}

int main() {
    int n;

    // Read input for the index of the term to compute
    printf("Enter a non-negative integer (0-99): ");
    scanf("%d", &n);

    // The behavior is undefined for invalid inputs, so we assume valid input as per the specification
    if (n >= 0 && n <= 99) {
        long long result = compute_nth_term(n);
        printf("The %dth term of the sequence is: %lld\n", n, result);
    }

    return 0;
}
