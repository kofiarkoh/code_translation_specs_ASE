// AOJ 3032 Combine Two Elements
// 2018.3.29 bal4u

#include <stdio.h>
#include <string.h>

//#define getchar_unlocked()  getchar()
int in()	/* 整数値の入力 */
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

#define MAX 1604
int  hi[MAX], to[MAX][MAX];
char seen[MAX];
int  match[MAX];

int bpm(int u);
int bipartiteMatching(int m, int n)
{
	int u, max;

	memset(match, -1, (m+n)*sizeof(int));
	max = 0;
	for (u = 0; u < m; u++) {
		memset(seen, 0, m+n);
		if (bpm(u)) max++;
	}
	return max;
}

int bpm(int u)
{
	int i, v;

	for (i = 0; i < hi[u]; i++) {
		v = to[u][i];
		if (seen[v]) continue;
		seen[v] = 1;
		if (match[v] < 0 || bpm(match[v])) {
			match[u] = v, match[v] = u;
			return 1;
		}
	}
	return 0;
}

#define ABS(a)  ((a)>=0?(a):-(a))

typedef struct { int a, b; } T;
T u[802]; int us;
T v[802]; int vs;

int main()
{
	int N, A, B, A2, i, j, a, b, d, ans;

	N = in(), A = in(), B = in(), A2 = A<<1;
	ans = 0, us = vs = 0;
	for (i = 0; i < N; i++) {
		a = in(), b = in();
		d = ABS(a-b);
		if (d <= A || (B <= d && d <= A2)) { ans++; continue; }
		if (a >= b) u[us].a = a, u[us++].b = b;
		else        v[vs].a = a, v[vs++].b = b;
	}

	for (i = 0; i < us; i++) for (j = 0; j < vs; j++) {
		a = u[i].a + v[j].a;
		b = u[i].b + v[j].b;
		d = ABS(a-b);
		if (d <= A || (B <= d && d <= A2)) to[i][hi[i]++] = us+j;
	}

	ans += bipartiteMatching(us, vs);
	printf("%d\n", ans);
	return 0;
}

