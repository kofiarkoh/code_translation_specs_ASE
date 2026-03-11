#include <stdio.h>

const int Mod = 1000000007;

int main()
{
	int N, K;
	scanf("%d %d", &N, &K);

	int i, j, m, n;
	long long dp[2][2][32000] = {}, num[32000], sum;
	for (n = 1; n * n <= N; n++) dp[1][0][n] = 1;
	n--;
	if (n * (n + 1) <= N) m = n;
	else m = n - 1;
	for (j = 1; j <= m; j++) {
		num[j] = N / j - N / (j + 1);
		dp[1][1][j] = num[j];
	}
	for (i = 2; i <= K; i++) {
		for (j = 1, sum = 0; j <= m; j++) {
			sum += dp[1-i%2][0][j];
			if (sum > Mod) sum -= Mod;
			dp[i%2][1][j] = sum * num[j];
			if (dp[i%2][1][j] > Mod) dp[i%2][1][j] %= Mod;
		}
		if (m == n) dp[i%2][0][n] = sum + dp[1-i%2][1][n];
		else dp[i%2][0][n] = sum + dp[1-i%2][0][n];
		if (dp[i%2][0][n] > Mod) dp[i%2][0][n] -= Mod;
		for (j = n - 1; j >= 1; j--) {
			dp[i%2][0][j] = dp[i%2][0][j+1] + dp[1-i%2][1][j];
			if (dp[i%2][0][j] > Mod) dp[i%2][0][j] -= Mod;
		}
	}
	
	long long ans = 0;
	for (j = 1; j <= n; j++) ans += dp[K%2][0][j];
	for (j = 1; j <= m; j++) ans += dp[K%2][1][j];
	printf("%lld\n", ans % Mod);
	fflush(stdout);
	return 0;
}