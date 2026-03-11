#include <stdio.h>
#include <stdlib.h>

#define N	100000
#define MD	1000000007

int vv[N * 2 + 1], ff[N * 2 + 1];

void init() {
	int i;

	ff[0] = 1;
	for (i = 1; i <= N * 2; i++) {
		vv[i] = i == 1 ? 1 : (long long) vv[i - MD % i] * (MD / i + 1) % MD;
		ff[i] = (long long) ff[i - 1] * i % MD;
	}
}

int *oj[N * 2], oo[N * 2];

void link(int i, int j) {
	int o = oo[i]++;

	if (o >= 2 && (o & o - 1) == 0)
		oj[i] = (int *) realloc(oj[i], o * 2 * sizeof *oj[i]);
	oj[i][o] = j;
}

int ta[N * 2], pp[N * 2], qu[N * 2], cnt, qu_[N * 2], cnt_;

int dfs1(int p, int i) {
	static int time;
	int o;

	ta[i] = ++time, pp[i] = p, qu[cnt++] = i;
	for (o = 0; o < oo[i]; o++) {
		int j = oj[i][o];

		if (j != p) {
			if (!ta[j]) {
				if (!dfs1(i, j))
					return 0;
			} else if (ta[j] < ta[i]) {
				int i_;

				if (cnt_ != 0)
					return 0;
				cnt_ = 0;
				for (i_ = j; i_ != -1; i_ = pp[i_])
					qu_[cnt_++] = i_;
				while (--cnt_)
					pp[qu_[cnt_]] = qu_[cnt_ - 1];
				pp[j] = i;
				for (i_ = i; i_ != j; i_ = pp[i_])
					qu_[cnt_++] = i_;
				qu_[cnt_++] = j;
			}
		}
	}
	return 1;
}

char visited[N * 2]; int sz[N * 2];

void dfs2(int i) {
	int o;

	if (visited[i])
		return;
	visited[i] = 1;
	sz[i] = 1;
	for (o = 0; o < oo[i]; o++) {
		int j = oj[i][o];

		if (j < pp[i]) {
			dfs2(j);
			sz[i] += sz[j];
		}
	}
}

int solve() {
	int h, x;

	for (h = 0; h < cnt; h++)
		visited[qu[h]] = 0;
	for (h = 0; h < cnt; h++)
		if (!visited[qu[h]])
			dfs2(qu[h]);
	x = 1;
	for (h = 0; h < cnt; h++)
		x = (long long) x * vv[sz[qu[h]]] % MD;
	return x;
}

int main() {
	int n, h, i, j, ans;

	init();
	scanf("%d", &n);
	for (i = 0; i < n * 2; i++)
		oj[i] = (int *) malloc(2 * sizeof *oj[i]);
	for (h = 0; h < n * 2; h++) {
		scanf("%d%d", &i, &j), i--, j--;
		link(i, n + j), link(n + j, i);
	}
	ans = ff[n * 2];
	for (i = 0; i < n * 2; i++)
		if (!ta[i]) {
			int x, y;

			cnt = 0, cnt_ = 0;
			if (!dfs1(-1, i) || cnt_ == 0) {
				printf("0\n");
				return 0;
			}
			x = solve();
			for (h = 0; h < cnt_; h++)
				pp[qu_[h]] = qu_[(h - 1 + cnt_) % cnt_];
			y = solve();
			ans = (long long) ans * (x + y) % MD;
		}
	printf("%d\n", ans);
	return 0;
}
