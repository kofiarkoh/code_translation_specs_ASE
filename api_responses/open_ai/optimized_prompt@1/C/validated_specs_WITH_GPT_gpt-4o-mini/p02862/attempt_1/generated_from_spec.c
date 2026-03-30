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

// Function to calculate the number of distinct paths
int calculate_paths(int x, int y) {
    // Check preconditions
    if (x < 0 || y < 0) return 0;

    // Check postconditions
    if ((x + y) % 3 != 0) return 0;

    int n = (x + y) / 3; // n is the number of steps to take
    int x_ = x - n; // adjusted x
    int y_ = y - n; // adjusted y

    // Check if adjusted coordinates are valid
    if (x_ < 0 || y_ < 0) return 0;

    // Calculate the number of distinct paths using combinatorial formula
    long long result = (factorial[x_ + y_] * mod_inverse(factorial[x_], MOD) % MOD * mod_inverse(factorial[y_], MOD) % MOD) % MOD;
    return (int)result;
}

int main() {
    // Initialize factorials
    initialize_factorials();

    // Example usage
    int x, y;
    printf("Enter coordinates (x y): ");
    scanf("%d %d", &x, &y);
    int paths = calculate_paths(x, y);
    printf("Number of distinct paths: %d\n", paths);
    return 0;
}
