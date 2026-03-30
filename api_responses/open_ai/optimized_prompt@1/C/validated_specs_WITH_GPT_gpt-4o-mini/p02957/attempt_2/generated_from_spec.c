#include <stdio.h>

int main() {
    long int a, b; // Declare two long integers for user input
    printf("Enter two long integers: ");
    
    // Read user input
    scanf("%ld %ld", &a, &b);
    
    long int sum = a + b; // Calculate the sum of the two integers
    
    // Check if the sum is even
    if (sum % 2 == 0) {
        long int average = sum / 2; // Calculate the average
        printf("%ld\n", average); // Output the average
    } else {
        printf("IMPOSSIBLE\n"); // Output "IMPOSSIBLE" if the sum is odd
    }
    
    return 0; // Indicate successful completion
}
