// AOJ DSL-DSL_2_H Range Query - RMQ and RAQ
// 2019.3.27 bal4u

#include <stdio.h>
#include <stdlib.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif
int in()
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

void out(int n)
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else {
		if (n < 0) pc('-'), n = -n;
		i = 0; while (n) ob[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

#define MAX (1<<17) //segment tree のサイズ。この実装では2べきにする必要がある
#define MIN(a, b)  ((a)<=(b)?(a):(b))
#define INF  0x1fffffff

int segMin[2*MAX], segAdd[2*MAX];

// 区間[a, b)に値xを加算する
// int k = 0, int l = 0, int r = MAX
void add(int a, int b, int x, int k, int l, int r)
{
	if (r <= a || b <= l) return;    //もし交差しない区間であれば終える.

	if (a <= l && r <= b) { // もし今みている区間[l, r)が[a, b)に完全に内包されていれば
		segAdd[k] += x;     // 区間[l, r)にkを加算
		return;
	}

	add(a, b, x, (k<<1) + 1, l, (l+r) >> 1);  // 子の区間に(必要があれば)xを加算する.
	add(a, b, x, (k<<1) + 2, (l+r) >> 1, r);  // 〃

	//親の区間の最小値は, 子の区間の最小値 + 自分に一様に加算されている値である.一様に加算される値は更新しなくて良い.
	segMin[k] = MIN(segMin[(k<<1) + 1] + segAdd[(k<<1) + 1], segMin[(k<<1) + 2] + segAdd[(k<<1) + 2]);
}

//  int k = 0, int l = 0, int r = MAX
int getMin(int a, int b, int k, int l, int r)
{
	int le, ri;

	if (r <= a || b <= l) return INF;
	if (a <= l && r <= b) return segMin[k] + segAdd[k]; // 完全に内包されていれば,その区間の最小値を返す.

	le = getMin(a, b, (k<<1)+1, l, (l+r) >> 1);   // 子の区間の最小値を求める.
	ri = getMin(a, b, (k<<1)+2, (l+r) >> 1, r);   // 子の区間の最小値を求める

	return MIN(le, ri) + segAdd[k]; // 親の区間の最小値は, 子の区間の最小値 + 自分に一様に加算されている値
}

int main()
{
	int N, Q;
	int cmd, s, t, x;

	N = in(), Q = in();
	while (Q--) {
		cmd = in(), s = in(), t = in() + 1;
		if (cmd == 0) {
			x = in();
			add(s, t, x, 0, 0, MAX);
		}
		else out(getMin(s, t, 0, 0, MAX));
	}
}
