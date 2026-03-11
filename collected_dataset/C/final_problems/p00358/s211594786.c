// AOJ 0363 Loading
// 2018.2.11 bal4u

#include <stdio.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

char b[20002];
int dp[20002][16];

int main()
{
	int h, n, x, y;
	int i, h1, ans;
	
	h = in(), n = in(), h1 = h-1;
	while (n--) {
		x = in();
		b[in()] |= 1 << x;
	}
	
	for (y = 0; y < h1; y++) for (x = 0; x < 16; x++) {
		if (dp[y+1][0] < dp[y][x]) dp[y+1][0] = dp[y][x];

		if (!b[y] && !b[y+1] && dp[y+1][15] < dp[y][x]+2) dp[y+1][15] = dp[y][x]+2;

		for (i = 0; i < 3; i++) {
			int mask = 3 << i;
			if (!(x & mask) && !(b[y] & mask) && !(b[y+1] & mask)) {
				if (dp[y+1][mask] < dp[y][x]+1) dp[y+1][mask] = dp[y][x]+1;
			}
		}
	}
	
	ans = 0, i = 16; while (i--) if (ans < dp[h1][i]) ans = dp[h1][i];
	printf("%d\n", ans);
	return 0;
}
