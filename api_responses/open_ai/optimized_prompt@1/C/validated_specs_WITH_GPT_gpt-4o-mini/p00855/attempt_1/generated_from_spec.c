#include <stdio.h>
#include <stdbool.h>

// Function to check if a number is prime
bool is_prime(int n) {
    if (n <= 1) return false; // 0 and 1 are not prime
    for (int i = 2; i * i <= n; i++) {
        if (n % i == 0) return false; // Found a divisor, not prime
    }
    return true; // No divisors found, it's prime
}

// Function to count prime numbers in the adjusted range
int count_primes(int lower, int upper) {
    // Decrement lower until a prime is found
    while (lower >= 0 && !is_prime(lower)) {
        lower--;
    }
    
    // Increment upper until a prime is found
    while (!is_prime(upper)) {
        upper++;
    }
    
    // Count primes in the inclusive range [lower, upper]
    int count = 0;
    for (int i = lower; i <= upper; i++) {
        if (is_prime(i)) {
            count++;
        }
    }
    
    return count; // Return the count of primes found
}

int main() {
    int lower, upper;
    
    // Read input values
    if (scanf("%d %d", &lower, &upper) != 2) {
        // Undefined behavior if input is not valid integers
        return 1; // Exit with error code
    }
    
    // Preconditions check: non-negative integers
    if (lower < 0 || upper < 0) {
        return 1; // Exit with error code
    }
    
    // Calculate and print the count of primes
    int prime_count = count_primes(lower, upper);
    printf("%d\n", prime_count);
    
    return 0; // Successful execution
}
