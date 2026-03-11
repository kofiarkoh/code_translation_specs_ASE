// AOJ 1086 Live Schedule
// 2018.2.4 bal4u

#include <stdio.h>
#include <string.h>

int E[31][16], F[31][16];
int dp[31][51][6];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
    int C, D, W, X, c, d, w, x, k, ans;
	int se, sf, t;

	while (C = in()) {
		D = in(), W = in(), X = in();

		for (c = 0; c < C; c++) for (d = 0; d < D; d++) E[d][c] = in();
		for (c = 0; c < C; c++) for (d = 0; d < D; d++) F[d][c] = in();

		for (d = 0; d < D; d++) {
			memcpy(dp[d+1], dp[d], sizeof(dp[d]));
			for (c = 0; c < C; c++) {
				se = sf = 0;
				for (k = c; k < C; k++) {
					if (!E[d][k]) break;
					se += E[d][k], sf += F[d][k];
					if (k == c) {
						for (w = sf; w <= W; w++) for (x = 0; x <= X; x++) {
							if ((t=dp[d][w-sf][x]+se) > dp[d+1][w][x]) dp[d+1][w][x] = t;
						}
					} else {
						for (w = sf; w <= W; w++) for (x = 0;  x < X; x++) {
							if ((t=dp[d][w-sf][x]+se) > dp[d+1][w][x+1]) dp[d+1][w][x+1] = t;
						}
					}
				}
			}
		}
		
		ans = 0;
		for (w = 0; w <= W; w++) for (x = 0; x <= X; x++)
			if (ans < dp[D][w][x]) ans = dp[D][w][x];
		printf("%d\n", ans);
	}
	return 0;
}
