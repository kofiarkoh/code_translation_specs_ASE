#include <stdio.h>
#include <limits.h> // For INT_MIN

int main() {
    int a, b;
    // Initialize max_value to a value less than any possible computed result
    int max_value = INT_MIN; 

    // Read two integers from input
    scanf("%d %d", &a, &b);

    // Compute sum, difference, and product
    int sum = a + b;
    int difference = a - b;
    int product = a * b;

    // Update max_value based on the computed values
    if (sum > max_value) {
        max_value = sum;
    }
    if (difference > max_value) {
        max_value = difference;
    }
    if (product > max_value) {
        max_value = product;
    }

    // Output the maximum value
    printf("%d\n", max_value);

    return 0;
}
