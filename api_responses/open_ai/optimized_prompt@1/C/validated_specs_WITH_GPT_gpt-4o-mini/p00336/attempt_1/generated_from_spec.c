#include <stdio.h>
#include <string.h>

#define MOD 1000000007
#define MAX_LEN 1001

// Function to compute the number of distinct subsequences of t that match b
int countDistinctSubsequences(const char *t, const char *b) {
    int n = strlen(t);
    int m = strlen(b);
    
    // If either string is empty, return 1 (empty string is a subsequence of any string)
    if (n == 0 || m == 0) {
        return 1;
    }

    // Dynamic programming table
    long long dp[MAX_LEN][MAX_LEN] = {0};

    // Initialize the first column: empty string b is a subsequence of any prefix of t
    for (int i = 0; i <= n; i++) {
        dp[i][0] = 1; // There's one way to form an empty subsequence
    }

    // Fill the DP table
    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= m; j++) {
            // If characters match, we can either include or exclude the character from t
            if (t[i - 1] == b[j - 1]) {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % MOD;
            } else {
                // If characters do not match, we can only exclude the character from t
                dp[i][j] = dp[i - 1][j] % MOD;
            }
        }
    }

    // The result is in the bottom-right cell of the DP table
    return (int)(dp[n][m] % MOD);
}

int main() {
    char t[MAX_LEN], b[MAX_LEN];

    // Read input strings
    scanf("%s", t);
    scanf("%s", b);

    // Check for length constraints (undefined behavior if exceeded)
    if (strlen(t) > MAX_LEN - 1 || strlen(b) > MAX_LEN - 1) {
        return 1; // Undefined behavior as per specification
    }

    // Compute and print the result
    int result = countDistinctSubsequences(t, b);
    printf("%d\n", result);

    return 0;
}
