// AOJ GRL_5_D Tree - Range Query on a Tree
// 2019.3.29 bal4u

#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>

//// 入力の高速化
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
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(int n)
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


//// BIT library
#define MAX 100005
int bit[2*MAX]; int bmax;

int sum(int i)
{
	int s = 0;

	while (i > 0) {
		s += bit[i];
		i -= i & -i;
	}
	return s;
}

void add(int i, int v)
{
	while (i <= bmax) {
		bit[i] += v;
		i += i & -i;
	}
}


//// 本問題関連
int N;
int hi[MAX], *to[MAX];
int idx[2*MAX][2]; int sz;
int que[2*MAX], top;

void eularTour()
{
	int i, node;

	sz = 1;

	// 非再帰dfs
	que[0] = 0, top = 1;
	while (top) {
		node = que[--top];

		// dfsにする
		if (node < 0) {
			node = -node;
			idx[node - 1][1] = sz++;
			continue;
		}
		que[top++] = -(node + 1);

		idx[node][0] = sz++;
		for (i = 0; i < hi[node]; i++) que[top++] = to[node][i];
	}

#if 0
	int j;
	for (j = 0; j <= 1; j++) {
		for (i = 0; i < sz; i++) printf("%3d ", idx[i][j]); printf("\n");
	}
#endif
}

int main()
{
	int i, j, k, Q;
	int cmd;

	N = in();
	for (i = 0; i < N; i++) {
		hi[i] = k = in();
		if (k > 0) {
			to[i] = malloc(sizeof(int)*k);
			for (j = 0; j < k; j++) to[i][j] = in();
		}
	}
	eularTour();
	bmax = sz + 1;

	Q = in();
	while (Q--) {
		cmd = in();
		if (cmd == 0) {  // add(v, w)
			int v, w;
			v = in(), w = in();
			add(idx[v][0], w);
			add(idx[v][1], -w);
		}
		else {           // getSum(u)
			int u = in();
			if (u == 0) out(0);
			else out(sum(idx[u][0]));
		}
	}
	return 0;
}
