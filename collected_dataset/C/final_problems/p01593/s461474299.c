// AOJ 2481 Earn Big
// 2018.2.7 bal4u
 
#include <stdio.h>

double dp[1002];

int main()
{
	int n, m, i, j;

	scanf("%d%d", &n, &m);
	dp[0] = 1;
	for (i = 0; i < n; i++) {
		j = 0;
		while (i+j < n && j < m) dp[i + ++j] += dp[i] / (n-i);
    }
	printf("%.8lf\n", dp[n]);
	return 0;
}

