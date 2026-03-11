#include <stdio.h>

long long int max(long long int a, long long int b) {
	if (a >= b) {
		return a;
	}
	return b;
}

int main(void) {
	int n, i;
	scanf("%d", &n);
	long long int a[n];
	for (i = 0; i < n; i++) {
		scanf("%lld", &a[i]);
	}

	long long int dp[n][2];
	dp[0][0] = 0;
	dp[0][1] = -999999999999999999;
	for (i = 0; i < n; i++) {
		dp[i + 1][0] = max(dp[i][0] + a[i], dp[i][1] - a[i]);
		dp[i + 1][1] = max(dp[i][0] - a[i], dp[i][1] + a[i]);
	}
	printf("%lld\n", dp[n][0]);
	return 0;
}
