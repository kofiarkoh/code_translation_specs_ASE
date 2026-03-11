#include <stdio.h>

int dp[60][60][2510] = {{{1}}};	// i個からj個選んで和をsにする選び方の総数

// O(N^3X)の解法
int main(void)
{
	int i, j, A, N, s, x;
	long long int ans = 0;

	scanf("%d%d", &N, &A);
	dp[0][0][0] = 1;
	for (i = 1; i <= N; i++) {
		scanf("%d", &x);
		for (j = 0; j <= N; j++) {
			for (s = 0; s <= 2500; s++) {
				if (s < x) {
					dp[i][j][s] = dp[i - 1][j][s];
				} else if (j >= 1 && s >= x) {
					dp[i][j][s] = dp[i - 1][j - 1][s - x]	// xを選択した場合
								+ dp[i - 1][j][s];			// xを選択しなかった場合
				} 
			}
		}
	}

	for (i = 1; i <= N; i++) {
		ans += dp[N][i][i * A];
	}

	printf("%lld\n", ans);

	return 0;
}