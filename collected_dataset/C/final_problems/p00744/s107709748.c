// AOJ 1163: Cards
// 2017.9.15 bal4u@uu
// 2018.3.12

#include <stdio.h>
#include <string.h>

#define MAX 1004
short hi[MAX], to[MAX][MAX];
char  seen[MAX];
short match[MAX];

int bipartiteMatching(int m, int n)
{
	int u, max;
	int bpm(int u);

	memset(match, -1, (m+n) << 1);
	max = 0;
	for (u = 0; u < m; u++) {
		if (match[u] < 0) {
			memset(seen, 0, m+n);
			if (bpm(u)) max++;
		}
	}
	return max;
}

int bpm(int u)
{
	int i, v, w;

	seen[u] = 1;
	for (i = 0; i < hi[u]; i++) {
		v = to[u][i], w = match[v];
		if (w < 0 || !seen[w] && bpm(w)) { match[u] = v, match[v] = u; return 1; }
	}
	return 0;
}

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

int gcd(int a, int b)
{
	int r;
if (!((a | b) & 1)) return 2;
	while (b != 0) r = a % b, a = b, b = r;
	return a;
}

int b[501], r[502];

int main()
{
	int i, j, m, n;

	while (m = in()) {
		n = in();
		for (i = 0; i < m; i++) b[i] = in();
		for (j = 0; j < n; j++) r[j] = in();

		memset(hi, 0, (m+n) << 2);
	    for (i = 0; i < m; i++) for (j = 0; j < n; j++) {
			if (gcd(b[i], r[j]) > 1) to[i][hi[i]++] = m + j;
		}			

		printf("%d\n", bipartiteMatching(m, n));
	}
	return 0;
}

