#include <stdio.h>

#define MOD 1000000007
#define MAX 1000000

// Function to compute factorial % MOD
long long factorial[MAX + 1];

// Function to compute modular inverse using Fermat's Little Theorem
long long mod_inverse(long long a, long long p) {
    long long result = 1;
    long long exp = p - 2; // p is prime
    while (exp) {
        if (exp % 2) {
            result = (result * a) % p;
        }
        a = (a * a) % p;
        exp /= 2;
    }
    return result;
}

// Function to initialize factorial and inverse factorial arrays
void initialize_factorials() {
    factorial[0] = 1;
    for (int i = 1; i <= MAX; i++) {
        factorial[i] = (factorial[i - 1] * i) % MOD;
    }
}

// Function to compute the number of distinct paths
int number_of_paths(int x, int y) {
    if (x < 0 || y < 0) return 0; // Check for negative inputs

    int sum = x + y;
    if (sum % 3 != 0) return 0; // Check divisibility by 3

    int n = sum / 3; // Calculate n
    int x_prime = x - n; // Calculate x'
    int y_prime = y - n; // Calculate y'

    // Check if adjusted coordinates are negative
    if (x_prime < 0 || y_prime < 0) return 0;

    // Calculate the number of distinct paths using combinations
    long long result = (factorial[x_prime + y_prime] * 
                        mod_inverse(factorial[x_prime], MOD) % MOD * 
                        mod_inverse(factorial[y_prime], MOD) % MOD) % MOD;

    return (int)result; // Return the result as an integer
}

int main() {
    initialize_factorials(); // Initialize factorials before usage

    int x, y;
    // Example input; in practice, you would get these from user input or other sources
    x = 6; // Example value for x
    y = 3; // Example value for y

    int paths = number_of_paths(x, y);
    printf("%d\n", paths); // Output the number of distinct paths

    return 0;
}
