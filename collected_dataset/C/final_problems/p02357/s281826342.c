// AOJ DSL_3_D Sliding Minimum Elements
// 2018.5.5 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else {
		i = 0; while (n) ob[i++] = n%10 + '0', n/=10;
		while (i--) pc(ob[i]);
	}
}

#define INF 0x50505050		// > 10^9

// セグメント木の初期化
int seg[1<<18]; int sz;

void segtree(int n, int init_val)
{
	sz = 2; while (sz < n) sz <<= 1;
	n = sz << 1;
	memset(seg+sz-1, INF, sz<<2);
//	for (i = 0; i < n; i++) seg[i] = init_val;
}

void update()
{
	int i, k, n, p, v;

	n = sz;	while (n > 0) {
		for (p = n-1, i = 0; i < n; i+=2, p+=2) {
			k = p >> 1;
			v = seg[p+1]; if (v > seg[p]) v = seg[p];
			seg[k] = v;
		}
		n >>= 1;
	}
}

int query2(int a, int b, int k, int l, int r) 
{
	int m, lmin, rmin;

	if (r <= a || b <= l) return INF;
    if (a <= l && r <= b) return seg[k];
	k <<= 1, m = (l+r) >> 1;
    lmin = query2(a, b, k+1, l, m);
    rmin = query2(a, b, k+2, m, r);
	if (lmin > rmin) lmin = rmin;
    return lmin; 
}

// 空間 [a, b] 内の最小値
int range_min_query(int a, int b)
{
	return query2(a, b+1, 0, 0, sz);
}


int main()
{
	int N, L, i, j;

	N = in(), L = in();
	segtree(N, INF);
	j = sz-1;
	for (i = 0; i < N; i++) seg[j++] = in();
	update();
	out(range_min_query(0, L-1));
	for (i = 1, j = L; i <= N-L; i++, j++) {
		pc(' '), out(range_min_query(i, j));
	}
	pc('\n');
	return 0;
}

