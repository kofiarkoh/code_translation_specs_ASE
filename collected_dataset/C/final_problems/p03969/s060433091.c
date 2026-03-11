#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N	50
#define M	100
#define K	100
#define MD	1000000007

int min(int a, int b) { return a < b ? a : b; }

int vv[M + K + 1], ff[M + K + 1], gg[M + K + 1], tt[M + 1], pp[M + 1];

void init(int k) {
	int i, j;

	ff[0] = gg[0] = 1;
	for (i = 1; i <= M + K; i++) {
		vv[i] = i == 1 ? 1 : (long long) vv[i - MD % i] * (MD / i + 1) % MD;
		ff[i] = (long long) ff[i - 1] * i % MD;
		gg[i] = (long long) gg[i - 1] * vv[i] % MD;
	}
	for (i = 1; i <= M; i++)
		tt[i] = i;
	for (i = 2; i <= M; i++) {
		if (tt[i] != i)
			continue;
		for (j = i; j <= M; j += i)
			tt[j] = tt[j] / i * (i - 1);
	}
	pp[0] = 1;
	for (i = 1; i <= M; i++)
		pp[i] = (long long) pp[i - 1] * k % MD;
}

int *oh[N], oo[N];

void link(int i, int h) {
	int o = oo[i]++;

	if (o >= 2 && (o & o - 1) == 0)
		oh[i] = (int *) realloc(oh[i], o * 2 * sizeof *oh[i]);
	oh[i][o] = h;
}

int ij[M], ds[M], ke[M];

int find(int i) {
	return ds[i] < 0 ? i : (ds[i] = find(ds[i]));
}

void join(int i, int j) {
	i = find(i);
	j = find(j);
	if (i == j)
		return;
	if (ds[i] > ds[j])
		ds[i] = j, ke[j] += ke[i];
	else {
		if (ds[i] == ds[j])
			ds[i]--;
		ds[j] = i, ke[i] += ke[j];
	}
}

int ta[N], tb[N];

void dfs(int f, int i) {
	static int time;
	int o;

	ta[i] = tb[i] = ++time;
	for (o = 0; o < oo[i]; o++) {
		int h = oh[i][o], j = i ^ ij[h];

		if (h != f) {
			if (!ta[j]) {
				dfs(h, j);
				tb[i] = min(tb[i], tb[j]);
				if (tb[j] < ta[i])
					join(h, f);
			} else if (ta[j] < ta[i]) {
				tb[i] = min(tb[i], ta[j]);
				join(h, f);
			}
		}
	}
}

int count(int n) {	/* https://en.wikipedia.org/wiki/Burnside%27s_lemma */
	int m, cnt;

	cnt = 0;
	for (m = 1; m <= n; m++)
		if (n % m == 0)
			cnt = (cnt + (long long) pp[m] * tt[n / m]) % MD;
	cnt = (long long) cnt * vv[n] % MD;
	return cnt;
}

int choose(int n, int k) {
	return (long long) ff[n] * gg[k] % MD * gg[n - k] % MD;
}

int main() {
	static int kv[M];
	static char used[M];
	int n, m, k, h, i, j, ans;

	scanf("%d%d%d", &n, &m, &k), init(k);
	for (i = 0; i < n; i++)
		oh[i] = (int *) malloc(2 * sizeof *oh[i]);
	for (h = 0; h < m; h++) {
		scanf("%d%d", &i, &j), i--, j--;
		ij[h] = i ^ j;
		link(i, h), link(j, h);
	}
	for (h = 0; h < m; h++)
		ds[h] = -1, ke[h] = 1;
	for (i = 0; i < n; i++)
		if (!ta[i])
			dfs(-1, i);
	for (i = 0; i < n; i++) {
		int o;

		for (o = 0; o < oo[i]; o++) {
			h = find(oh[i][o]);
			if (!used[h])
				used[h] = 1, kv[h]++;
		}
		for (o = 0; o < oo[i]; o++)
			used[find(oh[i][o])] = 0;
	}
	ans = 1;
	for (h = 0; h < m; h++)
		if (ds[h] < 0)
			ans = (long long) ans * (ke[h] == 1 ? k : (ke[h] == kv[h] ? count(ke[h]) : choose(ke[h] + k - 1, k - 1))) % MD;
	printf("%d\n", ans);
	return 0;
}
