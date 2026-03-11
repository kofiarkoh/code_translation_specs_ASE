#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

#define N	100000

unsigned int X;

void srand_() {
	struct timeval tv;

	gettimeofday(&tv, NULL);
	X = tv.tv_sec ^ tv.tv_usec;
}

int rand_() {
	return (X *= 3) >> 1;
}

int *oj[N], oo[N];

void link(int i, int j) {
	int o = oo[i]++;

	if (o >= 2 && (o & o - 1) == 0)
		oj[i] = (int *) realloc(oj[i], o * 2 * sizeof *oj[i]);
	oj[i][o] = j;
}

void sort(int *aa, int l, int r) {
	while (l < r) {
		int i = l, j = l, k = r, a = aa[l + rand_() % (r - l)], tmp;

		while (j < k)
			if (aa[j] == a)
				j++;
			else if (aa[j] < a) {
				tmp = aa[i], aa[i] = aa[j], aa[j] = tmp;
				i++, j++;
			} else {
				k--;
				tmp = aa[j], aa[j] = aa[k], aa[k] = tmp;
			}
		sort(aa, l, i);
		l = k;
	}
}

int solve(int *aa, int n, int i_, int l) {
	int i = 0, j = n - 1;

	while (i < j)
		if (i == i_)
			i++;
		else if (j == i_)
			j--;
		else {
			if (aa[i] + aa[j] > l)
				return 0;
			i++, j--;
		}
	return 1;
}

int dp[N];

int dfs(int p, int i, int l) {
	static int aa[N];
	int o, n, lower, upper;

	for (o = 0; o < oo[i]; o++) {
		int j = oj[i][o];

		if (j != p && !dfs(i, j, l))
			return 0;
	}
	n = 0;
	for (o = 0; o < oo[i]; o++) {
		int j = oj[i][o];

		if (j != p)
			aa[n++] = dp[j] + 1;
	}
	sort(aa, 0, n);
	if (p == -1)
		return solve(aa, n % 2 == 0 ? n : n - 1, -1, l);
	if (n % 2 == 0) {
		if (solve(aa, n, -1, l)) {
			dp[i] = 0;
			return 1;
		}
		n--;
	}
	lower = -1, upper = n;
	while (upper - lower > 1) {
		int h = (lower + upper) / 2;

		if (solve(aa, n, h, l))
			upper = h;
		else
			lower = h;
	}
	return upper < n && (dp[i] = aa[upper]) < l;
}

int main() {
	int n, h, i, j, k, lower, upper;

	srand_();
	scanf("%d", &n);
	for (i = 0; i < n; i++)
		oj[i] = (int *) malloc(2 * sizeof *oj[i]);
	for (h = 0; h < n - 1; h++) {
		scanf("%d%d", &i, &j), i--, j--;
		link(i, j), link(j, i);
	}
	k = n - 1;
	for (i = 0; i < n; i++)
		k -= oo[i] / 2;
	lower = 0, upper = n - 1;
	while (upper - lower > 1) {
		int l = (lower + upper) / 2;

		if (dfs(-1, 0, l))
			upper = l;
		else
			lower = l;
	}
	printf("%d %d\n", k, upper);
	return 0;
}
