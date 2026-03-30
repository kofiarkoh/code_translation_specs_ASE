#include <stdio.h>

long long compute_nth_term(int n) {
    // Preconditions: n must be a non-negative integer
    // Postconditions: Returns the nth term of the sequence

    // Handle the base cases
    if (n == 0) {
        return 2; // l[0] = 2
    } else if (n == 1) {
        return 1; // l[1] = 1
    }

    // Array to store the sequence values
    long long l[n + 1]; // Space complexity is O(n)

    // Initialize the first two terms
    l[0] = 2;
    l[1] = 1;

    // Compute the sequence iteratively
    for (int i = 2; i <= n; i++) {
        l[i] = l[i - 2] + l[i - 1]; // l[n] = l[n-2] + l[n-1]
    }

    // Return the nth term
    return l[n];
}

int main() {
    int n;
    printf("Enter a non-negative integer n: ");
    scanf("%d", &n); // Input for the index of the term

    // Call the function and print the result
    long long result = compute_nth_term(n);
    printf("The %dth term of the sequence is: %lld\n", n, result);

    return 0;
}
