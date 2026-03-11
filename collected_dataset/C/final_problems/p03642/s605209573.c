#include <stdio.h>
#include <stdlib.h>

#define N	100
#define A	10000000

char composite[A + 1];

void init() {
	int a, b;

	for (a = 2; a <= A / a; a++) {
		if (composite[a])
			continue;
		for (b = a * a; b <= A; b += a)
			composite[b] = 1;
	}
}

int *ov[1 + N], oo[1 + N];

void link(int u, int v) {
	int o = oo[u]++;

	if (o >= 2 && (o & o - 1) == 0)
		ov[u] = (int *) realloc(ov[u], o * 2 * sizeof *ov[u]);
	ov[u][o] = v;
}

int vv[1 + N], uu[1 + N], dd[1 + N];

int bfs(int n) {
	static int qu[1 + N];
	int u, head, cnt;

	for (u = 0; u <= n; u++)
		dd[u] = n;
	head = cnt = 0;
	for (u = 1; u <= n; u++)
		if (!vv[u])
			dd[u] = 0, qu[head + cnt++] = u;
	while (cnt) {
		int d, o;

		u = qu[cnt--, head++], d = dd[u] + 1;
		for (o = 0; o < oo[u]; o++) {
			int v = ov[u][o], w = uu[v];

			if (dd[w] == n) {
				dd[w] = d;
				if (w == 0)
					return 1;
				qu[head + cnt++] = w;
			}
		}
	}
	return 0;
}

int dfs(int u, int n) {
	int d, o;

	if (u == 0)
		return 1;
	d = dd[u] + 1;
	for (o = 0; o < oo[u]; o++) {
		int v = ov[u][o], w = uu[v];

		if (dd[w] == d && dfs(w, n)) {
			vv[u] = v, uu[v] = u;
			return 1;
		}
	}
	dd[u] = n;
	return 0;
}

int hopcroft_karp(int n) {
	int cnt = 0;

	while (bfs(n)) {
		int u;

		for (u = 1; u <= n; u++)
			if (!vv[u] && dfs(u, n))
				cnt++;
	}
	return cnt;
}

int main() {
	static int xx[N], xx0[N], xx1[N];
	int n, n0, n1, m, i, u, v;

	init();
	scanf("%d", &n);
	for (i = 0; i < n; i++)
		scanf("%d", &xx[i]);
	n0 = n1 = 0;
	for (i = 0; i < n; i++) {
		if (i == 0 || xx[i - 1] + 1 != xx[i]) {
			if (xx[i] % 2 == 0)
				xx0[n0++] = xx[i];
			else
				xx1[n1++] = xx[i];
		}
		if (i + 1 == n || xx[i] + 1 != xx[i + 1]) {
			if ((xx[i] + 1) % 2 == 0)
				xx0[n0++] = xx[i] + 1;
			else
				xx1[n1++] = xx[i] + 1;
		}
	}
	for (u = 1; u <= n0; u++)
		ov[u] = (int *) malloc(2 * sizeof *ov[u]);
	for (u = 0; u < n0; u++)
		for (v = 0; v < n1; v++) {
			int x = abs(xx0[u] - xx1[v]);

			if (x > 2 && !composite[x])
				link(1 + u, 1 + v);
		}
	m = hopcroft_karp(n0);
	printf("%d\n", m + (n0 + n1 - m * 2) + (n0 - m) % 2);
	return 0;
}
