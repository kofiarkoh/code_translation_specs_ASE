// AOJ 1395 What Goes Up Must Come Down
// bitツリーによる転倒数の計算
// 2019.3.25 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//// 数値入力
#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

//// Fenwick Tree
#define MAX 100005
int bit[MAX], maxn;
void add(int i, int v)
{
	while (i <= maxn) bit[i] += v, i += i & -i;
}

int _sum(int i)
{
	int s = 0;
	while (i > 0) s += bit[i], i -= i & -i;
	return s;
}

int sum(int left, int right) {
	return _sum(right) - _sum(left);
}

int n;
int a[MAX];
int re[MAX];

int main()
{
	int i, t;
	long long ans;

	n = in();
	maxn = 0;  for (i = 0; i < n; i++) {
		a[i] = in();
		if (maxn < a[i]) maxn = a[i];
	}
	maxn++;
	ans = 0;
	for (i = 0; i < n; i++) {
		re[i] = sum(a[i], maxn);
		add(a[i], 1);
	}

	memset(bit, 0, sizeof(bit));
	for (i = n - 1; i >= 0; i--) {
		t = sum(a[i], maxn);
		if (t < re[i]) ans += t; else ans += re[i];
		add(a[i], 1);
	}
	printf("%lld\n", ans);
	return 0;
}
