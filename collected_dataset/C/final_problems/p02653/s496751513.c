#include <stdio.h>
#include <stdlib.h>

#define Mod 1000000007

int main()
{
	int N, A, B;
	scanf("%d %d %d", &N, &A, &B);
	if (A > B) {
		B += A;
		A = B - A;
		B -= A;
	}

	int i, j, k;
	long long ans = 0, **dp[2];
	dp[0] = (long long**)malloc(sizeof(long long*) * B);
	dp[1] = (long long**)malloc(sizeof(long long*) * B);
	for (i = 0; i <= B - 1; i++) {
		dp[0][i] = (long long*)malloc(sizeof(long long) * (A + 1));
		dp[1][i] = (long long*)malloc(sizeof(long long) * (A + 1));
	}
	if (A == 1 || B == 1) {
		for (i = 1, ans = 1; i <= N; i++) ans = ans * 2 % Mod;
	} else if (A == B) {
		for (j = A - 2, dp[0][A%2][A-1] = 2; j >= 1; j--) dp[0][A%2][j] = dp[0][A%2][j+1] * 2 % Mod;
		for (i = A + 1, ans = 2; i <= N; i++) {
			ans = (ans * 2 + dp[0][1-i%2][A-1]) % Mod;
			for (j = A - 2, dp[0][i%2][1] = dp[0][1-i%2][A-1]; j >= 1; j--) {
				dp[0][i%2][j+1] = dp[0][1-i%2][j];
				dp[0][i%2][1] += dp[0][1-i%2][j];
			}
			dp[0][i%2][1] %= Mod;
		}
	} else {
		dp[1][1][0] = 1;
		dp[1][0][1] = 1;
		for (i = 2; i <= N; i++) {
			dp[i%2][0][1] = 0;
			ans = (ans * 2 + dp[1-i%2][B-1][0] + dp[1-i%2][B-A][A-1] + dp[1-i%2][B-A-1][A] * 2) % Mod;
			
			for (j = B - 2; j >= 1; j--) dp[i%2][j+1][0] = dp[1-i%2][j][0];
			for (j = B - 1, dp[i%2][B-A][1] = 0; j >= B - A; j--) dp[i%2][B-A][1] += dp[1-i%2][j][0];
			dp[i%2][B-A][1] %= Mod;
			for (; j >= 1; j--) dp[i%2][j][1] = dp[1-i%2][j][0];
			
			for (k = A - 1, dp[i%2][1][0] = 0; k >= 1; k--) {
				for (j = 0; j <= B - A; j++) {
					dp[i%2][j][k+1] = dp[1-i%2][j][k];
					dp[i%2][1][0] += dp[1-i%2][j][k];
				}
				dp[i%2][1][0] %= Mod;
			}
			for (j = 0; j <= B - A - 2; j++) {
				dp[i%2][j+1][A] = (dp[i%2][j+1][A] + dp[1-i%2][j][A]) % Mod;
				dp[i%2][j+A+1][0] = (dp[i%2][j+A+1][0] + dp[1-i%2][j][A]) % Mod;
			}
		}
	}
	
	printf("%lld\n", ans);
	fflush(stdout);
	return 0;
}