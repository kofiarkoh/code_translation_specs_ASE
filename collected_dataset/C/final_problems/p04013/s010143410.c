#include <stdio.h>

#define N 50
#define X 50

int main() {
	static long long dp[N + 1][N * X + 1];
	int n, a, h, i, j;
	long long cnt;

	scanf("%d%d", &n, &a);
	dp[0][0] = 1;
	for (h = 1; h <= n; h++) {
		int x;

		scanf("%d", &x);
		for (i = h; i > 0; i--)
			for (j = x; j <= i * X; j++)
				dp[i][j] += dp[i - 1][j - x];
	}
	cnt = 0;
	for (i = 1; i <= n; i++)
		cnt += dp[i][i * a];
	printf("%lld\n", cnt);
	return 0;
}
