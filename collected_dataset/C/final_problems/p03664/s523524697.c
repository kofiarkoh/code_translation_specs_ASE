#include <stdio.h>
#include <string.h>

#define N	15
#define INF	0x3f3f3f3f

int max(int a, int b) { return a > b ? a : b; }

int main() {
	static int ww[N][N], ss[1 << N], dp[1 << N][N];
	int n, m, i, j, b;

	scanf("%d%d", &n, &m);
	while (m--) {
		int w;

		scanf("%d%d%d", &i, &j, &w), i--, j--;
		ww[i][j] = ww[j][i] = w;
		ss[1 << i | 1 << j] += w;
	}
	for (i = 0; i < n; i++)
		for (b = 0; b < 1 << n; b++)
			if (b & 1 << i)
				ss[b] += ss[b ^ 1 << i];
	for (b = 1; b < 1 << n; b++)
		for (i = 0; i < n; i++)
			dp[b][i] = -INF;
	dp[1][0] = 0;
	for (b = 1; b < 1 << n; b++)
		for (i = 0; i < n; i++) {
			int x, c, b_;

			x = dp[b][i];
			if (x == -INF)
				continue;
			b_ = (1 << n) - 1 ^ b, c = 0;
			do
				dp[b | c][i] = max(dp[b | c][i], x + ss[c | 1 << i]);
			while (c = c - b_ & b_);
			for (j = 0; j < n; j++)
				if ((b & 1 << j) == 0 && ww[i][j])
					dp[b | 1 << j][j] = max(dp[b | 1 << j][j], x + ww[i][j]);
		}
	printf("%d\n", ss[(1 << n) - 1] - dp[(1 << n) - 1][n - 1]);
	return 0;
}
