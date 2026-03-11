#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N	300000
#define INF	0x3f3f3f3f

int max(int a, int b) { return a > b ? a : b; }

int xx[N], yy[N], yy_[N], n;

int compare_x(const void *a, const void *b) {
	int i = *(int *) a;
	int j = *(int *) b;

	return xx[i] - xx[j];
}

int compare_y(const void *a, const void *b) {
	int i = *(int *) a;
	int j = *(int *) b;

	return yy[i] - yy[j];
}

int ft1[N], ft2[N];

void update(int *ft, int i, int x) {
	while (i < n) {
		ft[i] = max(ft[i], x);
		i |= i + 1;
	}
}

int query(int *ft, int i) {
	int x = -INF;

	while (i >= 0) {
		x = max(x, ft[i]);
		i &= i + 1, i--;
	}
	return x;
}

void clear(int *ft, int i) {
	while (i < n) {
		ft[i] = -INF;
		i |= i + 1;
	}
}

int ii[N], jj[N];

void merge(int l, int m, int r) {
	int h1 = l, h2 = m, h = l;

	while (h1 < m && h2 < r)
		jj[h++] = yy[ii[h1]] < yy[ii[h2]] ? ii[h1++] : ii[h2++];
	while (h1 < m)
		jj[h++] = ii[h1++];
	while (h2 < r)
		jj[h++] = ii[h2++];
	memcpy(ii + l, jj + l, (r - l) * sizeof *jj);
}

int ll[N], rr[N];

int compare_l(const void *a, const void *b) {
	int i = *(int *) a;
	int j = *(int *) b;

	return ll[i] - ll[j];
}

int x_, y_, ans;

void solve(int l, int r) {
	static int qu[N], ii1[N], ii2[N];
	int m, h, h1, h2, cnt;

	if (r - l == 1)
		return;
	m = (l + r) / 2;
	solve(l, m);
	solve(m, r);
	cnt = 0;
	for (h = l; h < m; h++) {
		int i = ii[h];

		while (cnt && xx[qu[cnt - 1]] < xx[i])
			cnt--;
		ll[i] = cnt ? yy[qu[cnt - 1]] : -1;
		qu[cnt++] = i;
	}
	cnt = 0;
	for (h = m - 1; h >= l; h--) {
		int i = ii[h];

		while (cnt && xx[qu[cnt - 1]] < xx[i])
			cnt--;
		rr[i] = cnt ? yy[qu[cnt - 1]] : n;
		qu[cnt++] = i;
	}
	cnt = 0;
	for (h = m; h < r; h++) {
		int i = ii[h];

		while (cnt && xx[qu[cnt - 1]] > xx[i])
			cnt--;
		ll[i] = cnt ? yy[qu[cnt - 1]] : -1;
		qu[cnt++] = i;
	}
	cnt = 0;
	for (h = r - 1; h >= m; h--) {
		int i = ii[h];

		while (cnt && xx[qu[cnt - 1]] > xx[i])
			cnt--;
		rr[i] = cnt ? yy[qu[cnt - 1]] : n;
		qu[cnt++] = i;
	}
	for (h = l; h < m; h++)
		ii1[h - l] = ii[h];
	for (h = m; h < r; h++)
		ii2[h - m] = ii[h];
	qsort(ii1, m - l, sizeof *ii1, compare_l);
	qsort(ii2, r - m, sizeof *ii2, compare_l);
	for (h1 = 0, h2 = 0; h1 < m - l; h1++) {
		int i1, i2, x1, l1, r1;

		i1 = ii1[h1], x1 = xx[i1], l1 = ll[i1], r1 = rr[i1];
		while (h2 < r - m && ll[i2 = ii2[h2]] <= l1) {
			int x2 = xx[i2], r2 = rr[i2];

			update(ft1, r2, x2 + (r2 == n ? y_ : yy_[r2])), update(ft2, n - r2, x2);
			h2++;
		}
		ans = max(ans, max(query(ft1, r1 - 1) - x1 - (l1 == -1 ? 0 : yy_[l1]), query(ft2, n - r1) - x1 + (r1 == n ? y_ : yy_[r1]) - (l1 == -1 ? 0 : yy_[l1])));
	}
	for (h2 = 0; h2 < r - m; h2++) {
		int i2 = ii2[h2], r2 = rr[i2];

		clear(ft1, r2), clear(ft2, n - r2);
	}
	for (h2 = 0, h1 = 0; h2 < r - m; h2++) {
		int i1, i2, x2, l2, r2;

		i2 = ii2[h2], x2 = xx[i2], l2 = ll[i2], r2 = rr[i2];
		while (h1 < m - l && ll[i1 = ii1[h1]] <= l2) {
			int x1 = xx[i1], r1 = rr[i1];

			update(ft1, r1, -x1 + (r1 == n ? y_ : yy_[r1])), update(ft2, n - r1, -x1);
			h1++;
		}
		ans = max(ans, max(query(ft1, r2 - 1) + x2 - (l2 == -1 ? 0 : yy_[l2]), query(ft2, n - r2) + x2 + (r2 == n ? y_ : yy_[r2]) - (l2 == -1 ? 0 : yy_[l2])));
	}
	for (h1 = 0; h1 < m - l; h1++) {
		int i1 = ii1[h1], r1 = rr[i1];

		clear(ft1, r1), clear(ft2, n - r1);
	}
	merge(l, m, r);
}

int main() {
	int h, i;

	scanf("%d%d%d", &x_, &y_, &n);
	for (i = 0; i < n; i++) {
		scanf("%d%d", &xx[i], &yy[i]);
		ii[i] = i;
	}
	qsort(ii, n, sizeof *ii, compare_y);
	for (h = 0; h < n; h++)
		yy_[h] = yy[ii[h]], yy[ii[h]] = h;
	ans = max(yy_[0], y_ - yy_[n - 1]);
	for (h = 1; h < n; h++)
		ans = max(ans, yy_[h] - yy_[h - 1]);
	ans += x_;
	qsort(ii, n, sizeof *ii, compare_x);
	memset(ft1, -1, n * sizeof *ft1), memset(ft2, -1, n * sizeof *ft2);
	for (h = 0; h < n; h++) {
		int y, l, r;

		i = ii[h], y = yy[i], l = query(ft1, y), r = n - 1 - query(ft2, n - 1 - y);
		ans = max(ans, xx[i] + (r == n ? y_ : yy_[r]) - (l == -1 ? 0 : yy_[l]));
		update(ft1, y, y), update(ft2, n - 1 - y, n - 1 - y);
	}
	memset(ft1, -1, n * sizeof *ft1), memset(ft2, -1, n * sizeof *ft2);
	for (h = n - 1; h >= 0; h--) {
		int y, l, r;

		i = ii[h], y = yy[i], l = query(ft1, y), r = n - 1 - query(ft2, n - 1 - y);
		ans = max(ans, x_ - xx[i] + (r == n ? y_ : yy_[r]) - (l == -1 ? 0 : yy_[l]));
		update(ft1, y, y), update(ft2, n - 1 - y, n - 1 - y);
	}
	for (i = 0; i < n; i++)
		ft1[i] = ft2[i] = -INF;
	solve(0, n);
	ans *= 2;
	printf("%d\n", ans);
	return 0;
}
