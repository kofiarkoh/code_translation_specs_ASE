#include <stdio.h>

#define Mod 998244353

int main()
{
	int A, B, C, D;
	scanf("%d %d %d %d", &A, &B, &C, &D);
	if (A == C && B == D) {
		printf("1\n");
		fflush(stdout);
		return 0;
	}
	
	int i, j;
	long long dp[2][3002] = {};
	dp[0][0] = 1;
	for (j = 1; j <= D - B; j++) dp[0][j] = dp[0][j-1] * A % Mod;
	for (i = 1; i <= C - A; i++) {
		dp[i%2][0] = dp[1-i%2][0] * B % Mod;
		for (j = 1; j <= D - B; j++) dp[i%2][j] = (dp[i%2][j-1] * (A + i) + dp[1-i%2][j] * (B + j) - dp[1-i%2][j-1] * (A + i - 1) * (B + j - 1)) % Mod;
	}
	
	
	printf("%lld\n", dp[(C-A)%2][D-B]);
	fflush(stdout);
	return 0;
}