// AOJ 2449: Connect
// 2018.2.2 bal4u

#include <stdio.h>
#include <string.h>

char s[129][20]; int w[129];
int dp[2][65536], bc[65536];

int bitcount16(int x)
{
	x = ((x & 0xAAAAAA) >>  1) + (x & 0x555555);
	x = ((x & 0xCCCCCC) >>  2) + (x & 0x333333);
	x = ((x & 0xF0F0F0) >>  4) + (x & 0x0F0F0F);
	x = ((x & 0x00FF00) >>  8) + (x & 0xFF00FF);
	return x;
}

int main()
{
	int R, C, r, c, i, k, k1, k2, lim, p, q, x, ans;
	char buf[10], *t;

	fgets(buf, 10, stdin), sscanf(buf, "%d%d", &R, &C);
	for (r = 0; r < R; r++) {
		fgets(t=s[r], 20, stdin);
		while (*t > ' ') t++;
		w[r] = t - s[r];
	}
	lim = 1 << C;
	for (i = 0; i < lim; i++) bc[i] = bitcount16(i);
	
	memset(dp, -1, sizeof(dp));
	dp[0][0] = 0;
	
	k1 = 0, k2 = 1;
	for (r = 0; r < R; r++) for (c = 0; c < C; c++) {
		memset(dp[k2], -1, sizeof(dp[k2]));
		for (k = 0; k < lim; k++) if (dp[k1][k] >= 0) {
			p = bc[k & (1 << c) - 1];
			if (p < w[r]) {
				q = dp[k1][k];
				if (c && (k & (1 << (c-1))) && s[r][p-1] == s[r][p]) q++;
				if (r && (k & (1 << c))     && s[r][p  ] == s[r-1][w[r-1] - bc[k>>c]]) q++;
				x = k | (1 << c);
				if (q > dp[k2][x]) dp[k2][x] = q;
			}
			if (p + C - c > w[r]) {
				x = k & ~(1 << c);
				if (dp[k1][k] > dp[k2][x]) dp[k2][x] = dp[k1][k];
			}
		}
		k1 = k2, k2 = !k2;
	}

	ans = 0;
	for (c = 0; c < lim; c++) if (dp[k1][c] > ans) ans = dp[k1][c];
	printf("%d\n", ans << 1);
	return 0;
}
