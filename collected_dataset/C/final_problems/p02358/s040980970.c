// AOJ DSL_4_A Union of Rectangles
// 2018.5.14 bal4u

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
	if (c == '-') {	c = gc();
		do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
		return -n;
	}
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

#define MAX_N 2005
#define MAX   4005

int x1[MAX_N], y1[MAX_N], x2[MAX_N], y2[MAX_N];
int x[MAX], xs;
int y[MAX], ys;
short s[MAX][MAX];

int cmp(const void *a, const void *b) { return *(int *)a - *(int *)b; }

int xbsch(int a)
{
	int m, l = 0, r = xs;

    while (l < r) {
        m = (l + r) >> 1;
        if (x[m] < a) l = m + 1; else r = m;
    }
	return l;
}
int ybsch(int a)
{
	int m, l = 0, r = ys;

    while (l < r) {
        m = (l + r) >> 1;
        if (y[m] < a) l = m + 1; else r = m;
    }
	return l;
}

int uniq(int *a, int n)
{
	int i, j;
	
	for (i = 0, j = 1; j < n; j++) {
		while (j < n && a[j] == a[i]) j++;
		if (j < n) if (++i != j) a[i] = a[j];
	}
	return i+1;
}

int main()
{
	int N, i, r, c, r1, c1, r2, c2;
	long long ans;

	N = in(); xs = ys = 0;
	for (i = 0; i < N; i++) {
		x1[i] = c = in(), y1[i] = r = in();
		x[xs++] = c, y[ys++] = r;
		x2[i] = c = in(), y2[i] = r = in();
		x[xs++] = c, y[ys++] = r;
	}
	qsort(x, xs, sizeof(int), cmp); xs = uniq(x, xs);
	qsort(y, ys, sizeof(int), cmp); ys = uniq(y, ys);

	for (i = 0; i < N; i++) {
		c1 = xbsch(x1[i]), c2 = xbsch(x2[i]);
		r1 = ybsch(y1[i]), r2 = ybsch(y2[i]);

		s[r1][c1]++, s[r1][c2]--;
		s[r2][c1]--, s[r2][c2]++;
	}

	for (r = 0; r < ys; r++) for (c = 1; c < xs; c++) s[r][c] += s[r][c-1];
	for (r = 1; r < ys; r++) for (c = 0; c < xs; c++) s[r][c] += s[r-1][c];

	ans = 0;
	for (r = 0; r < ys; r++) for (c = 0; c < xs; c++) {
		if (s[r][c]) {
			c1 = c;
			while (s[r][c]) c++;
			ans += (long long)(x[c]-x[c1])*(y[r+1]-y[r]);
		}
	}
	printf("%lld\n", ans);
	return 0;
}

