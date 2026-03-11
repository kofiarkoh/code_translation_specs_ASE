// AOJ 2166: Erratic Sleep Habits
// 2018.1.4 bal4u@uu

#include <stdio.h>
#include <string.h>

int T;
int t[31];
char tbl[101];
int dp[101][101];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int calc(int d, int c)
{
	int i, k, c2, ans;

	if (d > 100) return 0;
	if (dp[d][c] >= 0) return dp[d][c];
	i = d+1;
	ans = 1+calc(i, 0), c2 = c;
	while (1) {
		if (i > 100) { ans = 0; break; }
		if (++c2 == T) c2 = 0;
		if (tbl[i] < t[c2]) break;
		k = 1+calc(++i, 0);
		if (k < ans) ans = k;
	}
	return dp[d][c] = ans;
}

int main()
{
	int N, d, m, i;

	while (T = in()) {
		for (i = 0; i < T; i++) t[i] = in();
		N = in();
		memset(tbl, 24, sizeof(tbl));
		for (i = 0; i < N; i++) {
			d = in(), m = in();
			if (m < tbl[d]) tbl[d] = m;
		}

		memset(dp, -1, sizeof(dp));
		printf("%d\n", calc(1, 0));
	}
	return 0;
}

