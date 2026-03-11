// AOJ DSL_2_D Range Update Query (RUQ)
// 2018.5.18 bal4u

#include <stdio.h>

#define INF 0x7fffffff

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

char *inf = "2147483647";

void out(int n)
{
	int i;
	char ob[20];

	if (!n) pc('0');
	else if (n == INF) for (i = 0; i < 10; i++) pc(inf[i]);
	else {
		i = 0; while (n) ob[i++] = n%10 + '0', n/=10;
		while (i--) pc(ob[i]);
	}
	pc('\n');
}

#define MAX_N 100005
#define INF 0x7fffffff

int node[MAX_N << 2];


int find(int t, int l, int r, int x)
{
	int m;

	if (~node[t]) return node[t];
	m = (l + r) >> 1;
	if (x <= m) return find( t<<1, l, m, x);
	else        return find((t<<1)|1, m + 1, r, x);
}

void update(int t, int l, int r, int x, int y, int k)
{
	int m;

	if (l == x && r == y) { node[t] = k; return; }
	m = (l + r) >> 1;
	if (~node[t]) {
		node[t<<1] = node[(t<<1)|1] = node[t];
		node[t] = -1;
	}
	if (y <= m)	update(t<<1, l, m, x, y, k);
	else if (x > m)	update((t<<1)|1, m+1, r, x, y, k);
	else {
		update(t<<1, l, m, x, m, k);
		update((t<<1)|1, m+1, r, m+1, y, k);
	}
}

signed main(void)
{
	int n, q, sz;
	int op, s, t, x;

	n = in(), q = in();
	sz = n << 2;
	for (s = 0; s < sz; s++) node[s] = INF;

	while (q--) {
		op = gc() & 1, gc();
		if (op) out(find(1, 1, n, in()+1));
		else {
			s = in()+1, t = in()+1, x = in();
			update(1, 1, n, s, t, x);
		}
	}
	return 0;
}
