#include <stdio.h>
#include <stdlib.h>

#define N	5000
#define MD	1000000007

int vv[N + 1], ff[N + 1], gg[N + 1];

long long choose(int n, int k) {
	return (long long) ff[n] * gg[k] % MD * gg[n - k] % MD;
}

void init() {
	int i;

	ff[0] = gg[0] = 1;
	for (i = 1; i <= N; i++) {
		vv[i] = i == 1 ? 1 : (long long) vv[i - MD % i] * (MD / i + 1) % MD;
		ff[i] = (long long) ff[i - 1] * i % MD;
		gg[i] = (long long) gg[i - 1] * vv[i] % MD;
	}
}

int *oj[N], oo[N];

void link(int i, int j) {
	int o = oo[i]++;

	if (o >= 2 && (o & o - 1) == 0)
		oj[i] = (int *) realloc(oj[i], o * 2 * sizeof *oj[i]);
	oj[i][o] = j;
}

int sz[N], n, c;

int dfs(int p, int i) {
	int o, centroid;

	sz[i] = 1;
	centroid = 1;
	for (o = 0; o < oo[i]; o++) {
		int j = oj[i][o];

		if (j != p) {
			if (dfs(i, j))
				return 1;
			if (sz[j] * 2 > n)
				centroid = 0;
			sz[i] += sz[j];
		}
	}
	if ((n - sz[i]) * 2 > n)
		centroid = 0;
	if (centroid) {
		if (c != -1)
			return 1;
		c = i;
	}
	return 0;
}

int main() {
	static int dp_[N + 1];
	int m, h, i, j, k, l, o, ans;

	init();
	scanf("%d", &n);
	for (i = 0; i < n; i++)
		oj[i] = (int *) malloc(2 * sizeof *oj[i]);
	for (h = 0; h < n - 1; h++) {
		scanf("%d%d", &i, &j), i--, j--;
		link(i, j), link(j, i);
	}
	c = -1;
	if (dfs(-1, 0)) {
		printf("%lld\n", (long long) ff[n / 2] * ff[n / 2] % MD);
		return 0;
	}
	dp_[0] = 1, m = 0;
	for (o = 0; o < oo[c]; o++) {
		int s;

		j = oj[c][o], s = sz[j] < sz[c] ? sz[j] : n - sz[c];
		for (k = m; k >= 0; k--) {
			int x = dp_[k];

			if (x == 0)
				continue;
			for (l = 1; l <= s; l++) {
				int c = choose(s, l);

				dp_[k + l] = (dp_[k + l] + (long long) c * c % MD * ff[l] % MD * x) % MD;
			}
		}
		m += s;
	}
	ans = 0;
	for (k = 0; k <= n; k++)
		ans = (ans + (long long) dp_[k] * ff[n - k] * (k % 2 == 0 ? 1 : -1)) % MD;
	if (ans < 0)
		ans += MD;
	printf("%d\n", ans);
	return 0;
}
