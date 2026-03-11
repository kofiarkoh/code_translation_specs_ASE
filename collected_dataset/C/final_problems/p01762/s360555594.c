// AOJ 2658: Pruning
// 2018.1.4 bal4u@uu

#include <stdio.h>

typedef struct { short c, p, ch, sib; char mk; } T;
T t[1002];

short to[1002][1002], cost[1002][1002];
short len[1002];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

void mktree(int par)
{
	int i, f, sib, tp;

	if (t[par].mk) return;
	t[par].mk = 1;

	f = 0;
	for (i = 0; i < len[par]; i++) {
		tp = to[par][i];
		if (t[tp].mk) continue;
		if (!f) t[par].ch = tp; 
		t[tp].p = cost[par][i];
		if (f) t[sib].sib = tp; else f = 1;
		sib = tp;
		mktree(sib);
	}
}

int calc(int par)
{
	int p, s, tp;

	p = t[par].p;
	if (t[par].c) return p;
	s = 0, tp = t[par].ch;
	while (tp) {
		s += calc(tp);
		if (s >= p) return p;
		tp = t[tp].sib;
	}
	return s;
}

int main()
{
	int n, u, v, p, i, k;

	n = in();
	for (i = 2; i <= n; i++) t[i].c = in();
	i = n-1; while (i--) {
		u = in()+1, v = in()+1, p = in();
		k = len[u]++, to[u][k] = v, cost[u][k] = p;
		k = len[v]++, to[v][k] = u, cost[v][k] = p;
	}
	mktree(1);
	t[1].p = 0x7fff;

	printf("%d\n", calc(1));
	return 0;
}
