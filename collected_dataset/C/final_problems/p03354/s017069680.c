/* Coached by rainboy */
#include <stdio.h>
#include <string.h>

#define N 100000
#define M 100000

int oo[1 + M * 2], oj[1 + M * 2];

int link(int o, int j) {
	static int _ = 1;

	oo[_] = o, oj[_] = j;
	return _++;
}

int ae[N], cc[N];
char visited[N];

void dfs(int i, int c) {
	int o;

	if (visited[i])
		return;
	visited[i] = 1;
	cc[i] = c;
	for (o = ae[i]; o; o = oo[o])
		dfs(oj[o], c);
}

int main() {
	static int ii[N + 1];
	int n, m, i, p, c, ans;

	scanf("%d%d", &n, &m);
	for (i = 0; i < n; i++) {
		scanf("%d", &p);
		ii[p] = i;
	}
	while (m--) {
		int x, y;

		scanf("%d%d", &x, &y), x--, y--;
		ae[x] = link(ae[x], y);
		ae[y] = link(ae[y], x);
	}
	memset(cc, -1, n * sizeof *cc);
	c = 0;
	for (i = 0; i < n; i++)
		if (!visited[i])
			dfs(i, c++);
	ans = 0;
	for (p = 1; p <= n; p++)
		if (cc[ii[p]] == cc[p - 1])
			ans++;
	printf("%d\n", ans);
	return 0;
}
