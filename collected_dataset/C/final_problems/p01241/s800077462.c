// AOJ 2119: Finding the Top RPS Player
// 2017.10.7 bal4u@uu

#include <stdio.h>
#include <string.h>

int dp[25];

int main()
{
	int t, n, m, i, s, cnt;

	t = 0; while (scanf("%d%d", &n, &m) && n > 0) {
		memset(dp, 0, sizeof(dp));
		dp[0] = n, cnt = 0;
		while (1) {
			s = 0;
			for (i = m; i >= 0; i--) {
				if (dp[i] >= 2) {
					dp[i+1] += dp[i] >> 1;
					s += dp[i] >> 1;
					dp[i] &= 1;
				}
			}
			dp[0] += s, cnt++;
			if (dp[m]) break;
		}
		printf("Case %d: %d\n", ++t, cnt);
	}
	return 0;
}