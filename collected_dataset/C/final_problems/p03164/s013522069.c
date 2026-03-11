#include <stdio.h>

const long long inf = 1e18;

int n, W;
int w[110], v[110];
long long dp[100100];

long long min(long long a, long long b) {
    return (a < b ? a : b);
}

int main() {
    scanf("%d%d", &n, &W);
    for (int i = 0; i < n; i++) scanf("%d%d", &w[i], &v[i]);

    for (int i = 0; i <= 100000; i++) dp[i] = inf;
    dp[0] = 0;
    for (int i = 0; i < n; i++) {
        for (int j = 100000; j >= v[i]; j--) {
            dp[j] = min(dp[j], dp[j - v[i]] + 1ll * w[i]);
        }
    }

    int ans = 0;
    for (int i = 0; i <= 100000; i++) {
        if (dp[i] <= W) ans = i;
    } 
    printf("%d\n", ans);
}