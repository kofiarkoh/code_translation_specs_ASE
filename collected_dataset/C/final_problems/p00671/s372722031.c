// AOJ 1086 Live Schedule
// 2018.2.4 bal4u

#include <stdio.h>
#include <string.h>

int E[31][16], F[31][16];
int dp[31][51][6];
int se[32], sf[32];

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
	int nw, nx, t;

	while (C = in()) {
		D = in(), W = in(), X = in();

		for (c = 0; c < C; c++) for (d = 0; d < D; d++) E[d][c] = in();
		for (c = 0; c < C; c++) for (d = 0; d < D; d++) F[d][c] = in();

		memset(dp, 0, sizeof(dp));
		for (d = 0; d < D; d++) {
			for (c = 0; c < C; c++) se[c+1] = se[c] + E[d][c], sf[c+1] = sf[c] + F[d][c];

			for (w = 0; w <= W; w++) for (x = 0; x <= X; x++) {
				if (dp[d+1][w][x] < dp[d][w][x]) dp[d+1][w][x] = dp[d][w][x];
				for (c = 0; c < C; c++) for (k = c+1; k <= C; k++) {
					if (!E[d][k-1]) break;
					nw = w + sf[k]-sf[c], nx = x + (k-c>1);
	                if (nw <= W && nx <= X && dp[d+1][nw][nx] < (t=dp[d][w][x]+se[k]-se[c]))
						dp[d+1][nw][nx] = t;
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
