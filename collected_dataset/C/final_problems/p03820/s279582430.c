#include <stdio.h>

#define N	2000
#define MD	1000000007

int main() {
	static int dp[N + 1];
	int n, k, i, j, ans;

	scanf("%d%d", &n, &k);
	dp[0] = 1;
	for (i = 0; i < k - 1; i++) {
		for (j = i + 1; j <= n; j++)
			dp[j] = (dp[j] + dp[j - 1]) % MD;
		dp[i] = 0;
	}
	ans = 0;
	for (j = 0; j < n; j++)
		ans = (ans + dp[j]) % MD;
	for (i = k; i < n - 1; i++)
		ans = ans * 2 % MD;
	printf("%d\n", ans);
	return 0;
}
