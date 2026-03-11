// AOJ 1333 Beautiful Spacing
// 2018.4.5 bal4u

#include <stdio.h>
#include <string.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

int  w, n;
int  s[50002];
char dp[50002];

int check(int x)
{
	int i, lim, f, cnt;

	lim = f = n, cnt = 0;
	memset(dp, 0, n);
	i = n; while (i--) {
		while (w-(s[lim]-s[i]) < lim-i-1) cnt -= dp[lim--];
		while (f > lim || f > i+1 && (w-(s[f]-s[i])+f-i-2) / (f-i-1) <= x)
			cnt += dp[f--];
		dp[i] = cnt > 0 || lim == n;
	}
	return dp[0];
}

int main()
{
	int i, lo, mi, hi;

	while (w = in()) {
		n = in();
		for (i = 1; i <= n; i++) s[i] = s[i-1] + in();
		lo = 0, hi = w;
		while (lo + 1 < hi) {
			mi = (lo + hi) >> 1;
			if (check(mi)) hi = mi; else lo = mi;
		}
		printf("%d\n", hi);
	}
	return 0;
}
