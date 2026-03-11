#include <stdio.h>

#define Mod 1000000007

int main()
{	
	int N, M;
	scanf("%d %d", &N, &M);
	
	int i, j, k;
	long long dp[2][302][302] = {};
	for (k = 1, dp[0][1][1] = 1; k <= M; k++) {
		for (i = 1; i <= N; i++) {
			for (j = 1; j <= i; j++) {				
				dp[k%2][i+1][j] += dp[1-k%2][i][j] * (N - i);
				dp[k%2][i][i] += dp[1-k%2][i][j] * j;
				dp[k%2][i][j] += dp[1-k%2][i][j] * (i - j);
				
				dp[1-k%2][i][j] = 0;
				dp[k%2][i][j] %= Mod;
			}
		}
	}
	
	printf("%lld\n", dp[M%2][N][N]);
	fflush(stdout);
	return 0;
}