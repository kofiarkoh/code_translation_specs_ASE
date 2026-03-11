// AOJ 2028: Gather on the Clock
// 2017.11.03 bal4u@uu

#include <stdio.h>
#include <string.h>

int a[102];
int dp[102][102];

int main()
{
	int cno, n, i, j, k, s, t, v, ans;
	
	scanf("%d", &cno); while (cno--) {
		scanf("%d", &n);
		for (i = 0; i < n; i++) scanf("%d", a+i);
		memset(dp, 0, sizeof(dp));

		for (k = 1; k < n; k++) for (i = 0; i < n; i++) for (s = i, t = k-1, j = 0; j < k; j++, t--) {
			if (++s == n) s = 0;
			v = a[s] - a[i]; if (v < 0) v = -v;
			v += dp[s][t] + dp[i][j];
			if (v > dp[i][k]) dp[i][k] = v;
		}

		for (ans = 0, i = 0; i < n; i++) if (dp[i][n-1] > ans) ans = dp[i][n-1];
		printf("%d\n",ans);
	}
	return 0;
}