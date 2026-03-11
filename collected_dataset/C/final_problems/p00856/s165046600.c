// AOJ 1277: Minimal Backgammon
// 2017.10.16 bal4u@uu

#include <stdio.h>
#include <string.h>

char tbl[102];
double dp[102][102];

int main()
{
	int n, t, l, b, i, j, k, a;
	double ans;

	while (scanf("%d%d%d%d", &n, &t, &l, &b) && n > 0) {
	    memset(tbl, 0, sizeof(tbl));
		memset(dp,0,sizeof(dp));
	    for (i = 0; i < l; i++) scanf("%d", &a), tbl[a] = 1;
		for (i = 0; i < b; i++) scanf("%d", &a), tbl[a] = 2;

		dp[0][0] = 1;
		for (i = 1; i <= t; i++) for (j = 0; j < n; j++) {
			for (k = 0; k < 6; k++) {
				if (dp[i-1][j] == 0) continue;
		        a = j + k+1;
		        if (a > n) a -= 2*(a-n);
				if      (tbl[a] == 0) dp[  i][a] += dp[i-1][j]/6;
				else if (tbl[a] == 1) dp[i+1][a] += dp[i-1][j]/6;
				else                  dp[  i][0] += dp[i-1][j]/6;
			}
		}
		for (ans = 0, i = 1; i <= t; i++) ans += dp[i][n];
		printf("%.6f\n",ans);
    }
	return 0;
}