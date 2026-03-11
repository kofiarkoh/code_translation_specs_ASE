#include <stdio.h>

int main()
{
	int i, L, A[200001];
	scanf("%d", &L);
	for (i = 1; i <= L; i++) scanf("%d", &(A[i]));
	
	int j;
	const long long sup = (long long)1 << 60;
	long long dp[2][5], tmp[5];
	for (j = 0; j < 5; j++) dp[0][j] = 0;
	for (i = 1; i <= L; i++) {
		for (j = 1; j < 5; j++) if (dp[1-i%2][j-1] < dp[1-i%2][j]) dp[1-i%2][j] = dp[1-i%2][j-1];
		dp[i%2][0] = dp[1-i%2][0] + A[i];
		dp[i%2][1] = dp[1-i%2][1] + ((A[i] == 0)? 2: A[i] % 2);
		dp[i%2][2] = dp[1-i%2][2] + 1 - A[i] % 2;
		dp[i%2][3] = dp[1-i%2][3] + ((A[i] == 0)? 2: A[i] % 2);
		dp[i%2][4] = dp[1-i%2][4] + A[i];
	}
	
	long long min = sup;
	for (j = 0; j < 5; j++) if (dp[1-i%2][j] < min) min = dp[1-i%2][j];
	printf("%lld\n", min);
	fflush(stdout);
	return 0;
}