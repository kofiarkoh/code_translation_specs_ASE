// Aizu 2305: Beautiful Currency
// 2017.10.30 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <math.h>

int a[22];
double dp[22][150001];

int main()
{
	int n, i, j, k;
	double ans;

	scanf("%d", &n);
	for (i = 0; i < n; i++) scanf("%d", a+i);

	memset(dp, 0x77, sizeof(dp)); dp[0][1] = 0;
    for (i = 0; i < n; i++) for (j = 1; j < 150000; j++) {
		for (k = j; k < 150000; k += j) {
			double x = fabs((double)(a[i] - k) / a[i]);
			if (dp[i][j] > x) x = dp[i][j];
			if (x < dp[i+1][k]) dp[i+1][k] = x;
		}
	}
	ans = 1.0;
	for (i = 0; i <= 15000; i++) if (dp[n][i] < ans) ans = dp[n][i]; 
	printf("%.12lf\n", ans);
	return 0;
}