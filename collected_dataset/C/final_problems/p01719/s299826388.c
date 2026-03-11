// AOJ 2607: Invest Master
// 2017.10.8 bal4u@uu

#include <stdio.h>

int p[12][12];
int dp[10002];

int main()
{
	int n, d, x, i, j, k, t;

	scanf("%d%d%d", &n, &d, &x);
	for (i = 0; i < d; i++) for (j = 0; j < n; j++) scanf("%d", &p[i][j]);
	for (i = 0; i < d-1; i++) {
		for (j = 0; j <= x; j++) dp[j] = j;
		for (j = 0; j < n; j++) for (k = 0; k <= x; k++) if (k >= p[i][j]) {
			t = dp[k-p[i][j]] + p[i+1][j];
			if (t > dp[k]) dp[k] = t;
		}
		for (t = 0, j = 0; j <= x; j++) if (dp[j] > t) t = dp[j];
		x = t;
	}
	printf("%d\n", x);
	return 0;
}