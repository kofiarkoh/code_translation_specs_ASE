// AOJ 0352: Forecast of Forces
// 2017.10.26

#include <stdio.h>

#define ABS(a) ((a)>=0?(a):-(a))
#define MAX(a,b) ((a)>=(b)?(a):(b))

int w, h;
long long map[1002][1002];
long long dp[1002][1002];

long long score(int r, int c)
{
	if ((r+c) & 1) return map[r+1][c] - (map[r+1][w] - map[r+1][c]);
	return (map[r+1][w] - map[r+1][c]) - map[r+1][c];
}

int main()
{
	int r, c, v;

	scanf("%d%d", &w, &h);
	for (r = 1; r <= h; r++) for (c = 1; c <= w; c++) {
		scanf("%d", &v);
		map[r][c] = map[r][c-1] + v;
	}
	for (r = h; r >= 0; r--) for (c = w; c >= 0; c--) {
		if (r == h) dp[r][c] = 0;
		else if (c == w) dp[r][c] = score(r, c) - dp[r+1][c];
		else dp[r][c] = MAX(-dp[r][c+1], score(r, c) - dp[r+1][c]);
	}
	printf("%lld\n", ABS(dp[0][0]));
	return 0;
}