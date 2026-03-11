// AOJ 2352 Divisor
// 2018.3.24 bal4u

#include <stdio.h>
#include <string.h>

#define MAX 202
int   hi[MAX],  to[MAX][MAX];
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

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

int  x[102]; int N;
char f[102];

int calc()
{
	int i, j;

	memset(hi, 0, N << 2);
	for (i = 0; i < N; i++) if (f[i]) {
		for (j = 0; j < N; j++) if (j != i && f[j]) {
			if (x[i] % x[j] == 0) to[i][hi[i]++] = N+j;
		}
	}
	return bipartiteMatching(N, N);
}

int main()
{
	int i, j, k, ans;
	int a1, a2, c;

	N = in();
	for (i = 0; i < N; i++) x[i] = in();
	memset(f, 1, N);
	ans = N - calc();

	j = 0; for (i = 0; i < N && j < ans; i++) {
		memset(f, 0, N), f[i] = 1, c = 1;
		for (k = 0; k < N; k++) if (x[k] % x[i] && x[i] % x[k]) f[i] = 1, c++;
		if (c < ans) continue;
		a1 = calc();
		memset(f, 1, N), f[i] = 0;
		a2 = calc();
		if (a1 <= a2) {
			if (j > 0) putchar(' ');
			printf("%d", i+1), j++;
		}
	}
	putchar('\n');
	return 0;
}

