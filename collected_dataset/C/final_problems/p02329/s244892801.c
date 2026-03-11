// AOJ DPL_4_A Coin Combination Problem
// 2018.5.9 bal4u
 
#include <stdio.h>
#include <string.h>
 
// 数値のハッシュ関数
#define HASHSIZ 3456781
typedef struct { long long v; int f; } HASH;
HASH hash[HASHSIZ+3], *hashend = hash+HASHSIZ;

int lookup(long long n)
{
	HASH *p = hash + (int)(n % HASHSIZ);
	while (p->v) {
		if (p->v == n) return p->f;
		if (++p == hashend) p = hash;
	}
	return 0;
}

void insert(long long n)
{
	HASH *p = hash + (int)(n % HASHSIZ);
	while (p->v) {
		if (p->v == n) { p->f++; return; }
		if (++p == hashend) p = hash;
	}
	p->v = n, p->f = 1;
}

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
 
int in()
{
    int n = 0, c = gc();
    do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
    return n;
}

long long inLL()
{
    long long n = 0, c = gc();
    do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
    return n;
}

long long a[1002], b[1002], c[1002], d[1002];

int main()
{
	int N, i, j;
	long long V, t, ans;

	N = in(), V = inLL();
	for (i = 0; i < N; i++) a[i] = inLL();
	for (i = 0; i < N; i++) b[i] = inLL();
	for (i = 0; i < N; i++) c[i] = inLL();
	for (i = 0; i < N; i++) d[i] = inLL();

	for (i = 0; i < N; i++) {
		if (a[i]+3 > V) continue;
		for (j = 0; j < N; j++) {
			t = a[i] + b[j];
			if (t+2 <= V) insert(t);
		}
	}

	ans = 0;
	for (i = 0; i < N; i++) {
		if (c[i]+3 > V) continue;
		for (j = 0; j < N; j++) {
			t = c[i] + d[j];
			if (t+2 <= V) ans += lookup(V-t);
		}
	}
	printf("%lld\n", ans);
	return 0;
}
