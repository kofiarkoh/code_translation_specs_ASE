#include <stdio.h>
#include <stdlib.h>

int a[300002], g[300002];

#define gc() getchar()
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int cmp(const void *a, const void *b)
{
	return *(int *)a - *(int *)b;
}

int main()
{
	int i, N, Q, x, y, t, xx, yy;
	int cnt;

	N = in();
	for (i = 1; i <= N; i++) g[i] = a[i] = in();
	qsort(g+1, N, sizeof(int), cmp);
	for (cnt = 0, i = 1; i <= N; i++) {
		if (g[i] != a[i]) cnt++;
	}
	if (cnt == 0) {
		puts("0");
		return 0;
	}
	Q = in();
	for (i = 1; i <= Q; i++) {
		x = in(), y = in();
		xx = a[x], yy = a[y];
		t = a[x], a[x] = a[y], a[y] = t;
		if (xx == g[x] && a[x] != g[x]) cnt++;
		else if (xx != g[x] && a[x] == g[x]) cnt--;
		if (yy == g[y] && a[y] != g[y]) cnt++;
		else if (yy != g[y] && a[y] == g[y]) cnt--;
		if (cnt == 0) {
			printf("%d\n", i);
			break;
		}
	}
	if (cnt) puts("-1");
	return 0;
}

