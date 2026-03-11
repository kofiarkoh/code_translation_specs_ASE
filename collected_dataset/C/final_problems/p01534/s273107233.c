// AOJ 2420: Anipero 2012
// 2017.12.30 bal4u@uu

#include <stdio.h>

int a[52], b[52], c[52];
int dp[52][52][10];
char mk[52][52][10];

int main()
{
	int n, m, i, j, k, u, v, w, s, ans;

	scanf("%d%d", &n, &m);
	for (i = 0; i < n; i++) scanf("%d%d%d", a+i,  b+i, c+i);

	dp[0][m][0] = 0, mk[0][m][0] = 1;
	for (i = 0; i < n; i++) for (j = 0; j <= m; j++) for (k = 0; k <= 8; k++) if (mk[i][j][k]) {
		for (u = 0; u <= k; u++) {
			int t, r;
			t = 8 - u; if (j < t) t = j;
			r = 8;     if (j < r) r = j;
			for (v = 0; v <= t; v++) {
				if (!u && !v) s = c[i];
				else s = v*a[i] + u*b[i];
				for (w = v; w <= r; w++) {
					int x = dp[i][j][k] + s;
					if (!mk[i+1][j-w][w] || x > dp[i+1][j-w][w]) 
						dp[i+1][j-w][w] = x, mk[i+1][j-w][w] = 1;
				}
			}
		}
	}
	ans = -0x7fffff;
	for (j = 0; j <= m; j++) for (k = 0; k <= 8; k++)
		if (mk[n][j][k] && dp[n][j][k] > ans) ans = dp[n][j][k];
	printf("%d\n", ans);
	return 0;
}