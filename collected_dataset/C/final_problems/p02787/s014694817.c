// ABC 153-E
// 2020.1.26 bal4u

#include <stdio.h>

#define gc() getchar_unlocked()

int in() {   // 非負整数の入力
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf); while ((c = gc()) >= '0');
	return n;
}

#define INF 0x5fffffff
int N;
int dp[20010];

int main()
{
	int i, H, ans;
	
	H = in(), N = in();
	for (i = 1; i < 20010; ++i) dp[i] = INF;
	while (N--) {
		int A = in(), B = in();
		for (i = 0; i <= H; ++i) if (dp[i] < INF) {
			if (dp[i] + B < dp[i+A]) dp[i+A] = dp[i] + B;
		}
	}
	ans = INF;
	for (i = H; i <= H+10000; ++i) if (dp[i] < ans) ans = dp[i];
	printf("%d\n", ans);
	return 0;
}
