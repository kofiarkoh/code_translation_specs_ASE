// AOJ DSL_2_I Range Query - RSQ and RUQ
// 2019.3.28 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//// 入力の高速化
#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif
int in()    // 整数の入力（負数対応）
{
	int n = 0, c = gc();
	if (c == '-') {
		c = gc();
		do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
		return -n;
	}
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(long long n)
{
	int i;
	char ob[40];

	if (!n) pc('0');
	else {
		if (n < 0) pc('-'), n = -n;
		i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}


// セグメント木
#define MAXN (1<<17)      // セグメント木のサイズ。2のべき乗にすべし
#define INF  0x1f1f1f1f

long long segVal[2*MAXN];
int segLazy[2*MAXN];

void lazy(int k, int l, int r)
{
	if (segLazy[k] != INF) {
		segVal[k] = (long long)(r-l)*segLazy[k];
		if (l + 1 < r) {
			segLazy[(k << 1) + 1] = segLazy[k];
			segLazy[(k << 1) + 2] = segLazy[k];
		}
		segLazy[k] = INF;
	}
}

void update(int a, int b, int x, int k, int l, int r)
{
	lazy(k, l, r);
	if (r <= a || b <= l) return;
	if (a <= l && r <= b) {
		segLazy[k] = x;
		lazy(k, l, r);
		return;
	}
	update(a, b, x, (k << 1) + 1, l, (l + r) >> 1);
	update(a, b, x, (k << 1) + 2, (l + r) >> 1, r);
	segVal[k] = segVal[(k << 1) + 1] + segVal[(k << 1) + 2];
}

long long getSum(int a, int b, int k, int l, int r)
{
	long long ans;

	if (r <= a || b <= l) return 0;
	lazy(k, l, r);
	if (a <= l && r <= b) return segVal[k];
	ans = getSum(a, b, (k << 1) + 1, l, (l + r) >> 1);
	ans += getSum(a, b, (k << 1) + 2, (l + r) >> 1, r);
	return ans;
}

int main()
{
	int N, Q;
	int cmd, s, t, x;

	memset(segLazy, INF, sizeof(segLazy));
	N = in(), Q = in();
	while (Q--) {
		cmd = in(), s = in(), t = in();
		if (cmd == 0) {
			x = in();
			update(s, t+1, x, 0, 0, N);
		}
		else out(getSum(s, t+1, 0, 0, N));
	}
	return 0;
}
