#include <stdio.h>
#include <stdlib.h>

#define N	100000
#define M	100000
#define INF	0x3f3f3f3f

int min(int a, int b) { return a < b ? a : b; }
int max(int a, int b) { return a > b ? a : b; }

int ds[N], rr[N];

int find(int i) {
	return ds[i] < 0 ? i : (ds[i] = find(ds[i]));
}

int join(int i, int j) {
	i = find(i);
	j = find(j);
	if (i == j)
		return 0;
	if (ds[i] > ds[j]) {
		ds[i] = j;
		rr[j] = rr[i];
	} else {
		if (ds[i] == ds[j])
			ds[i]--;
		ds[j] = i;
	}
	return 1;
}

int aa[N], bb[N];
int ii[M], jj[M];

int compare(const void *a, const void *b) {
	int h1 = *(int *) a;
	int h2 = *(int *) b;
	int i1 = ii[h1];
	int i2 = ii[h2];

	return aa[i1] != aa[i2] ? aa[i1] - aa[i2] : i1 - i2;
}

int main() {
	static int hh[M], ss[N], dp[N];
	int n, m, h, i, j;
	long long ans;

	scanf("%d%d", &n, &m);
	for (i = 0; i < n; i++) {
		scanf("%d%d", &aa[i], &bb[i]);
		aa[i] = max(aa[i] - bb[i], 0);
	}
	for (h = 0; h < m; h++) {
		int tmp;

		scanf("%d%d", &i, &j), i--, j--;
		if (aa[i] < aa[j] || aa[i] == aa[j] && i < j)
			tmp = i, i = j, j = tmp;
		ii[h] = i, jj[h] = j, hh[h] = h;
	}
	qsort(hh, m, sizeof *hh, compare);
	for (i = 0; i < n; i++) {
		ds[i] = -1, rr[i] = i;
		dp[i] = aa[i], ss[i] = bb[i];
	}
	for (h = 0; h < m; h++) {
		i = ii[hh[h]], j = rr[find(jj[hh[h]])];
		if (join(i, j)) {
			dp[i] = min(dp[i], max(dp[j], aa[i] - ss[j]));
			ss[i] = min(ss[i] + ss[j], INF);
		}
	}
	ans = dp[rr[find(0)]];
	for (i = 0; i < n; i++)
		ans += bb[i];
	printf("%lld\n", ans);
	return 0;
}
