// AOJ 1567 Lucky Number
// 2018.3.18 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// 高速数値入出力
#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif
int in()  // 非負整数のみ
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)   // 非負整数のみ, 最後改行
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else {
		i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

#define MOD 1000000007
int N, M, K;
int dp[5002][5002];
long long t[15010];

int main()
{
	int i, j, k, Q, a, b, c;
	int N2, K2;

	N = in(), M = in(), K = in(), Q = in();
	N2 = N << 1, K2 = K << 1;
	k = (N/K2)*K2; if (N % K2) k += K2;
	dp[0][0] = 1;
	for (i = 0; i < M; i++) {
		memset(t, 0, sizeof(t));
		for (j = 0; j < N; j++) {
			t[j] += dp[i][j];
			t[j + K] -= dp[i][j];
			if (j + k < N2) t[j + k] -= dp[i][j];
			if (j + k + K < N2) t[j + k + K] += dp[i][j];
		}
		
		for (j = 0; j < N2; j++) if (j + 1 < N2) t[j + 1] += t[j];
		for (j = 0; j < N2; j++) if (j + K2 < N2) t[j + K2] += t[j];
		for (j = 0; j < N; j++) dp[i + 1][j] = (t[j] + t[j + N]) % MOD;
	}

	while (Q--) {
		a = in(), b = in(), c = in() - a;
		if (c < 0) c += N;
		out(dp[b][c]);
	}
	return 0;
}
