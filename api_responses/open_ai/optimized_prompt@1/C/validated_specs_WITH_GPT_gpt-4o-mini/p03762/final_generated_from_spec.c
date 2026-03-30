#include <stdio.h>

#define MOD 1000000007

int main() {
    int n, m;
    
    // Read the number of elements in the two sequences
    scanf("%d %d", &n, &m);
    
    // Declare arrays for the sequences, using 1-based indexing
    int x[n + 1], y[m + 1];
    
    // Read the first sequence
    for (int i = 1; i <= n; i++) {
        scanf("%d", &x[i]);
    }
    
    // Read the second sequence
    for (int j = 1; j <= m; j++) {
        scanf("%d", &y[j]);
    }
    
    // Calculate the first derived value t
    long long t = 0; // Use long long to handle potential large sums
    for (int i = 1; i <= n; i++) {
        t = (t + ((i - 1) * (long long)x[i] - (n - i) * (long long)x[i]) % MOD + MOD) % MOD) % MOD;
    }
    
    // Calculate the second derived value k
    long long k = 0; // Use long long to handle potential large sums
    for (int j = 1; j <= m; j++) {
        k = (k + ((j - 1) * (long long)y[j] - (m - j) * (long long)y[j]) % MOD + MOD) % MOD) % MOD;
    }
    
    // Calculate the final result as the product of t and k, reduced modulo MOD
    long long result = (t * k) % MOD;
    
    // Output the result
    printf("%lld\n", result);
    
    return 0;
}
