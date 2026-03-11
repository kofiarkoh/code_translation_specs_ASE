// AOJ 2787 Pipe Fitter and the Fierce Dogs
// 2018.4.15 bal4u

#include <stdio.h>
#include <string.h>

// バッファを経ずstdinから数値を得る
//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

#define INF 0x10101010
int dp[10002];
char dog[5002][10002]; int max[5002];
 
int main()
{
	int W, H, K, N, x, y, s, w, lim, ans;

	W = in(), H = in(), K = in(), N = in();
	H >>= 1;
	K -= (W>>1)+1; if (K < 0) { puts("-1"); return 0; }
	while (N--) {
		x = in()-1, y = in()-1;
		if (y & 1) {
			y >>= 1;
			dog[y][x] = 1;
			if (x > max[y]) max[y] = x;
		}
	}

	s = 0, w = (W+1)<<2;
	for (y = 0; y < H; y++) {
		memset(dp, INF, w);
		dp[0] = 0, dp[1] = dog[y][0], dp[2] = dp[1];
		lim = max[y]+3; if (lim > W) lim = W;
		for (x = 2; x < lim; x++) {
			if (x & 1) dp[x+1] = dp[x];
			else {
				int t1 = dp[x] + dog[y][x];
				int t2 = dp[x-2] + (dog[y][x-1]<<1);
				if (t1 > t2) t1 = t2;
				if (dp[x+1] > t1) dp[x+1] = t1;
			}
		}
		s += dp[lim];
	}
	ans = H*((W>>1)+1) + s;
	if (s > K) s = K;
	ans -= s<<1, K -= s;
	if (ans > K) ans -= K; else ans = 0;
	printf("%d\n", ans);
	return 0;
}
