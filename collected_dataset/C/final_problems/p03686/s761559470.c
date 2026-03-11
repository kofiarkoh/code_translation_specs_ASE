#include <stdio.h>
#include <stdlib.h>

#define N	200000
#define M	200000
#define N_	(1 << 18)	/* N_ = pow2(ceil(log2(M))) */

int max(int a, int b) { return a > b ? a : b; }

int ll[N], rr[N];

int compare(const void *a, const void *b) {
	int i = *(int *) a;
	int j = *(int *) b;

	return ll[i] - ll[j];
}

int sum[N_ * 2], suf[N_ * 2], n_;

void pul(int i) {
	int l = i << 1, r = i << 1 | 1;

	sum[i] = sum[l] + sum[r];
	suf[i] = max(suf[r], suf[l] + sum[r]);
}

void build(int n) {
	int i;

	n_ = 1;
	while (n_ < n)
		n_ <<= 1;
	for (i = 1; i < n - 1; i++)
		sum[n_ + i] = -1;
	for (i = n_ - 1; i > 0; i--)
		pul(i);
}

void update(int i) {
	i += n_;
	suf[i] = max(sum[i]++, 0);
	while (i > 1)
		pul(i >>= 1);
}

int main() {
	static int ii[N];
	int n, m, i, l, ans;

	scanf("%d%d", &n, &m);
	for (i = 0; i < n; i++) {
		scanf("%d%d", &ll[i], &rr[i]);
		ii[i] = i;
	}
	qsort(ii, n, sizeof *ii, compare);
	build(m + 2);
	ans = max(n - m, 0);
	for (l = 0, i = 0; l <= m; l++) {
		while (i < n && ll[ii[i]] == l)
			update(rr[ii[i++]]);
		ans = max(ans, suf[1] - l);
	}
	printf("%d\n", ans);
	return 0;
}
