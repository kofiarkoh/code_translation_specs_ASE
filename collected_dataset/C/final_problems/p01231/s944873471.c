// AOJ 2108 Aaron and Bruce
// 2018.3.4

#include <stdio.h>
#include <string.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	while (c <= ' ') c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int m, lim;
int hi[51], to[51][51];
int dp[51][51][102];

int dfs(int a, int b, int t)
{
	int i, x, ans;

	if (a == b) return t;
	if (t >= lim) return lim;
	if (dp[a][b][t] >= 0) return dp[a][b][t];
	if (t & 1) {
		ans = lim;
		for (i = 0; i < hi[b]; i++) {
			x = dfs(a, to[b][i], t+1);
			if (x < ans) ans = x;
		}
	} else {
		ans = 0;
		for (i = 0; i < hi[a]; i++) {
			x = dfs(to[a][i], b, t+1);
			if (x > ans) ans = x;
		}
	}
	return dp[a][b][t] = ans;
}

int main()
{
	int cno, r, c, a, b, ans;

	cno = in(); while (cno--) {
		m = in(), lim = m << 1;
		for (r = 0; r < m; r++) {
			hi[r] = 1, to[r][0] = r;
			for (c = 0; c < m; c++)	if (in()) to[r][hi[r]++] = c;
		}
		a = in(), b = in();

		memset(dp, -1, sizeof(dp));
		ans = dfs(a, b, 0);
		if (ans >= lim)	puts("infinity");
		else printf("%d\n", ans >> 1);
	}
	return 0;
}
