// AOJ 2784 Similarity of Subtrees
// 2018.4.15 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// 数値のハッシュ関数
#define HASHSIZ 500009
typedef struct { unsigned long long id; int c; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash+HASHSIZ;
int  many;

void insert(unsigned long long id)
{
	HASH *p = hash + (int)(id % HASHSIZ);
	while (p->id) {
		if (p->id == id) { many = 1, p->c++; return; }
		if (++p == hashend) p = hash;
	}
	p->id = id, p->c = 1;
}

// バッファを経ずstdinから数値を得る
//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

#define M   1000000007
#define MAX 100002
int hi[MAX], *to[MAX];

long long dfs(int node)
{
	int i;
	unsigned long long ans;
	
	ans = 1;
	for (i = 0; i < hi[node]; i++) ans += (long long)dfs(to[node][i]) * M;
	insert(ans);
	return ans;
}

int main()
{
	int N, a, b, i, j;
	long long ans;
	int *memo;

	N = in();
	memo = malloc(N<<3);
	j = 0; for (i = 1; i < N; i++) {
		memo[j++] = a = in(), memo[j++] = b = in();
		hi[a]++;
	}
	for (i = 1; i <= N; i++) if (hi[i]) to[i] = malloc(hi[i]<<2);
	memset(hi, 0, N<<2);
	j = 0; for (i = 1; i < N; i++) {
		a = memo[j++], b = memo[j++];
		to[a][hi[a]++] = b;
	}

	dfs(1);

	ans = 0;
	if (many) for (i = 0; i < HASHSIZ; i++) {
		if ((j = hash[i].c) > 1) ans += (long long)j*(j-1) >> 1;
	}
	printf("%lld\n", ans);
	return 0;
}
