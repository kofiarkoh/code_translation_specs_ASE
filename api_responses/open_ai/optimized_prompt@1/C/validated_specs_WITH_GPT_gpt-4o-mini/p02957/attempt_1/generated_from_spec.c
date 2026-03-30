#include <stdio.h>

int main() {
    int a, b; // Declare two integers for user input
    // Prompt user for input
    printf("Enter two integers: ");
    scanf("%d %d", &a, &b); // Read two integers from user input

    int sum = a + b; // Calculate the sum of the two integers

    // Check if the sum is even
    if (sum % 2 == 0) {
        // If even, compute and print the average
        int average = sum / 2; // Integer division for average
        printf("%d\n", average); // Output the average
    } else {
        // If odd, print "IMPOSSIBLE"
        printf("IMPOSSIBLE\n");
    }

    return 0; // Indicate successful completion
}
