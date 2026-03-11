// AOJ 3045 Painting
// 2019.8.13 bal4u

#include <stdio.h>

typedef long long ll;

//// 入出力関係
#if 1
#define gc() getchar_unlocked()
#define pc(x) putchar_unlocked(x)
#else
#define gc() getchar()
#define pc(x) putchar(x)
#endif

int in() { // 非負整数の入力
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void out(ll n) { // 非負整数の表示（出力）
	int i; char b[50];

	if (!n) pc('0');
	else {
		i = 0; while (n) b[i++] = n % 10 + '0', n /= 10;
		while (i--) pc(b[i]);
	}
	pc('\n');
}

#define HASHSIZ 1000003 
typedef struct { ll k, id; } HASH;
HASH hash[HASHSIZ+5], *hashend = hash + HASHSIZ;

int insert(int d, int b, int id) {
	ll k = ((ll)d << 17) | b;
	HASH *p = hash + (int)(k % HASHSIZ);
	while (p->k) {
		if (p->k == k) return p->id;
		if (++p == hashend) p = hash;
	}
	p->k = k, p->id = id;
	return -1;
}

//// 本問題関連
typedef struct { int d, b, f; } T;
T tbl[100005]; int sz;
ll x[100005]; int w;
ll base;

int main()
{
	int i, j, a, b, N, M;

	N = in(), M = in();
	for (i = 0; i < M; i++) {
		a = in(), b = in();
		if (b == 1) continue;
		base += a, w++;
		if (b-a > N) continue;
		if ((j = insert(b-a, b, sz)) >= 0) tbl[j].f++;
		else tbl[sz].d = b-a, tbl[sz].b = b, tbl[sz++].f = 1;
	}
	for (i = 0; i < sz; i++) {
		for (j = tbl[i].d; j <= N; j += tbl[i].b) x[j] -= (ll)tbl[i].f * tbl[i].b;
	}
	for (i = 1; i <= N; i++) {
		x[i] += x[i-1] + w;
		out(x[i] + base);
	}
	return 0;
}

