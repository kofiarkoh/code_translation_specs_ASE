// AOJ 2944 Equal Split
// 2019.8.8 bal4u

#include <stdio.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif

int in() {   // 非負整数の入力
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf); while ((c = gc()) >= '0');
	return n;
}

void out(int n) { // 正整数の表示（出力）
	int i;
	char b[20];
	i = 0; while (n) b[i++] = n % 10 + '0', n /= 10;
	while (i--) pc(b[i]);
	pc('\n');
}

int main()
{
	int i, a, N, M, ans;
	
	while (N = in()) {
		M = in(), M /= N;
		ans = 0;
		for (i = 0; i < N; i++) {
			a = in();
			ans += (a >= M)? M: a;
		}
		out(ans);
	}
	return 0;
}
