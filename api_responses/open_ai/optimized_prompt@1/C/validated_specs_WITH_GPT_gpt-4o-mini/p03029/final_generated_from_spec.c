#include <stdio.h>

int compute_value(int a, int p) {
    // Compute the result based on the specified formula
    return (a * 3 + p) / 2; // Integer division automatically rounds down
}

int main() {
    int a, p;

    // Read two integers from input
    // The behavior is undefined if the input operation fails
    scanf("%d %d", &a, &p);

    // Compute the result using the specified function
    int result = compute_value(a, p);

    // Output the result
    printf("%d\n", result);

    return 0;
}
