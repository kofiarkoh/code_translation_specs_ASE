// AOJ 1049 Building Houses
// 2018.3.13 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

#define INF  10000000
int n;
int a[12][12];
int ans;
int id[12], pos[12]; char f[12];

void rec(int k, int len)
{
	int i, j, d;

	if (k >= n) {
		if (len < ans) ans = len;
		return;
	}
	for (i = 0; i < n; i++) if (!f[i]) {
		f[i] = 1, id[k] = i;
		d = a[id[k-1]][i] + len;
		for (j = 0; j < k; j++) {
			if (d - pos[j] < a[id[j]][i]) d = a[id[j]][i] + pos[j];
		}
		if (d < ans) { pos[k] = d, rec(k + 1, d); }
		f[i] = 0;
	}
}

int main()
{
	int i, r, c;

	while (n = in()) {
		for (r = 0; r < n; r++) for (c = 0; c < n; c++) a[r][c] = in();
		for (r = 0; r < n; r++) for (c = r + 1; c < n; c++) {
			if (a[c][r] > a[r][c]) a[r][c] = a[c][r];
			else a[c][r] = a[r][c];
		}
		if (n == 1) { puts("0"); continue; }

		ans = INF;
		memset(f, 0, sizeof(f));
		for (i = 0; i < n-1; i++) {
			id[0] = i, f[i] = 1, pos[0] = 0;
			rec(1, 0);
			f[i] = 0;
		}
		printf("%d\n", ans);
	}
	return 0;
}
