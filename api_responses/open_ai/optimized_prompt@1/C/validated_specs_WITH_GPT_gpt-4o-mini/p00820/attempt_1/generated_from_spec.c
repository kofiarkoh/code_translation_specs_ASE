#include <stdio.h>

int countRepresentations(int n, int maxSquare) {
    // Base case: if n is 0, there's one way to represent it (using no squares)
    if (n == 0) {
        return 1;
    }
    // If n is negative or no squares left to use, return 0
    if (n < 0 || maxSquare < 0) {
        return 0;
    }

    // Count representations including the current square and excluding it
    return countRepresentations(n - maxSquare * maxSquare, maxSquare) +
           countRepresentations(n, maxSquare - 1);
}

int main() {
    int n;

    // Read input until a zero is entered
    while (1) {
        printf("Enter a positive integer (0 to exit): ");
        scanf("%d", &n);

        // Termination condition
        if (n == 0) {
            break;
        }

        // Start counting representations using the largest square less than or equal to n
        int maxSquare = 1;
        while (maxSquare * maxSquare <= n) {
            maxSquare++;
        }
        maxSquare--; // Adjust to the largest valid square

        // Calculate and print the number of representations
        int result = countRepresentations(n, maxSquare);
        printf("Number of distinct representations of %d: %d\n", n, result);
    }

    return 0;
}
