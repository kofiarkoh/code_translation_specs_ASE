// Aizu 2305: Beautiful Currency
// 2017.10.30 bal4u@uu

#include <stdio.h>
#include <string.h>
#include <math.h>

#define MAX 150000
int a[22];
double dp[22][MAX+2];

int main()
{
	int n, i, j, k;
	double ans;

	scanf("%d", &n);
	for (i = 0; i < n; i++) scanf("%d", a+i);

	memset(dp, 0x77, sizeof(dp));
	for (j = 1; j < MAX; j++) dp[0][j] = fabs((double)(a[0] - j))/a[0];

    for (i = 0; i < n; i++) for (j = 1; j < MAX; j++) {
		for (k = j; k < MAX; k += j) {
			double x = fabs((double)(a[i] - k))/a[i];
			if (dp[i][j] > x) x = dp[i][j];
			if (x < dp[i+1][k]) dp[i+1][k] = x;
		}
	}

	ans = 1.0;
	for (j = 1; j < MAX; j++) if (dp[n][j] < ans) ans = dp[n][j]; 
	printf("%.12lf\n", ans);
	return 0;
}