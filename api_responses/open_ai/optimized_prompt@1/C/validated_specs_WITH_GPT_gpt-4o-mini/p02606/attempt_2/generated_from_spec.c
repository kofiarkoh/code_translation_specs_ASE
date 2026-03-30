#include <stdio.h>

int count_divisible(int L, int R, int d) {
    // Calculate the count of integers divisible by d in the range [L, R]
    // Using the formula: count = floor(R/d) - floor((L-1)/d)
    
    // Calculate the number of multiples of d up to R
    int count_R = R / d; // This is equivalent to floor(R / d)
    
    // Calculate the number of multiples of d up to L-1
    int count_L_minus_1 = (L - 1) / d; // This is equivalent to floor((L - 1) / d)
    
    // The count of integers divisible by d in the range [L, R]
    return count_R - count_L_minus_1; // This will always be non-negative due to preconditions
}

int main() {
    int L, R, d;
    
    // Example input; in a real scenario, these would be provided by the user
    L = 10; // Lower bound
    R = 30; // Upper bound
    d = 5;  // Divisor
    
    // Calculate and print the count of divisible integers
    int result = count_divisible(L, R, d);
    printf("Count of integers divisible by %d in range [%d, %d]: %d\n", d, L, R, result);
    
    return 0;
}
