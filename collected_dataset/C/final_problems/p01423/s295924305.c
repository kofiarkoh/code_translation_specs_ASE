// AOJ 2306 Rabbit Party
// 2018.4.8 bal4u

#include <stdio.h>

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

int n;
int rel[102][102];
int hi[102];
int id[102];
int ans;

void calc(int v, int d)
{
	int i, j, t, c, max, sum;

	id[d] = v;
	max = 14, sum = 0;
	for (i = 0; i <= d; i++) {
		if (hi[id[i]] < max) max = hi[id[i]];
		if (!d) break;
		c = 1000000;
		for (j = 0; j <= d; j++) if (i != j) {
			if ((t=rel[id[i]][id[j]]) < c) c = t;
		}
		sum += c;
	}
	if (d && sum > ans) ans = sum;
	if (d == max) return;
	for (i = v+1; i < n; i++) {
		if (hi[i] <= d) continue;
		for (j = 0; ; j++) {
			if (j == d) { calc(i, d+1); break; }
			if (!rel[i][id[j]]) break;
		}
	}
}

int main()
{
	int m, i, u, v;

	n = in(), m = in();
	while (m--) {
		u = in()-1, v = in()-1;
		hi[u]++, hi[v]++;
		rel[u][v] = rel[v][u] = in();
	}
	ans = 0;
	for (i = 0; i < n; i++) calc(i, 0);
	printf("%d\n", ans);
	return 0;
}
