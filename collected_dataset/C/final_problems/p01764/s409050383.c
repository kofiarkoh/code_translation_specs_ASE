// AOJ 2660: Sigma
// 2018.1.25 bal4u

#include <stdio.h>
#include <string.h>

#define INF 0x10101010

int s[302];
int dp[302][302];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int calc(int x, int y)
{
	int xi, yi, c, tm;

	tm = 0, c = 0;
	while (x || y) {
		xi = x % 10, x /= 10;
		yi = y % 10, y /= 10;
		c = (xi + yi + c) >= 10;
		tm += xi * yi + c;
	}
	return tm;
}

int main()
{
	int n, i, j, k, w, x;

	n = in();
	for (i = 0; i < n; i++) s[i+1] = s[i] + in();

	memset(dp, INF, sizeof(dp));
	for (i = 0; i < n; i++) dp[i][i] = 0;

	for (w = 1; w < n ; w++) {
		for (i = 0 ; i < n - w; i++) {
			j = i + w;
			for (k = i; k < j; k++) {
				x = dp[i][k] + calc(s[k+1]-s[i], s[j+1]-s[k+1]) + dp[k+1][j];
				if (x < dp[i][j]) dp[i][j] = x;
			}
		}
	}
	printf("%d\n", dp[0][n-1]);
	return 0;
}

