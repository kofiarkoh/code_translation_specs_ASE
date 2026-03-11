// ABC 157-D
// 2020.3.1 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)

int in() {   // 非負整数の入力
	int n = 0; int c;
	c = gc();
	do n = 10 * n + (c & 0xf); while ((c = gc()) >= '0');
	return n;
}

void out(int n) { // 非負整数の表示（出力）
	int i; char b[30];

	if (!n) pc('0');
	else {
		//		if (n < 0) pc('-'), n = -n;
		i = 0; while (n) b[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(b[i]);
	}
}

#define MAX 100005

/* UNION-FIND library */
int id[MAX], size[MAX];
void init(int n) { int i; for (i = 0; i < n; i++) id[i] = i, size[i] = 1; }
int root(int i) { while (i != id[i]) id[i] = id[id[i]], i = id[i]; return i; }
int connected(int p, int q) { return root(p) == root(q); }
void unite(int p, int q) {
    int i = root(p), j = root(q); if (i == j) return;
    if (size[i] < size[j]) id[i] = j, size[j] += size[i]; else id[j] = i, size[i] += size[j];
}

int N; // グラフの頂点数
int hi[MAX], *to[MAX];
int x[MAX];

void getGraph1(int M) {
	int i, k, a, b, c;
	int *memo, sz;

	init(N+1);
	memo = malloc(M*2*sizeof(int));
	sz = 0; for (i = 0; i < M; ++i) {
		a = in(), b = in();
		unite(a, b);
		hi[a]++, hi[b]++;
		memo[sz++] = a, memo[sz++] = b;
	}
	for (i = 1; i <= N; ++i) if (hi[i]) {
		to[i] = malloc(hi[i]*sizeof(int));
	}
	memset(hi, 0, sizeof(hi));

	i = 0; while (i < sz) {
		a = memo[i++], b = memo[i++];
		k = hi[a]++, to[a][k] = b;
		k = hi[b]++, to[b][k] = a;
	}
	free(memo);
}

int main()
{
	int i, M, K;

	N = in(), M = in(), K = in();
	getGraph1(M);
	while (K--) {
		int c = in(), d = in();
		if (connected(c, d)) x[c]++, x[d]++;
	}
	out(size[root(1)]-1 - hi[1]-x[1]);
	for (i = 2; i <= N; ++i) {
		pc(' '), out(size[root(i)]-1 - hi[i]-x[i]);
	}
	pc('\n');
	return 0;
}
