#include <stdio.h>
#include <stdlib.h>

#define N	200000

int *oj[N], oo[N];

void link(int i, int j) {
	int o = oo[i]++;

	if (o >= 2 && (o & o - 1) == 0)
		oj[i] = (int *) realloc(oj[i], o * 2 * sizeof *oj[i]);
	oj[i][o] = j;
}

int aa[N];

void dfs(int i) {
	static char used[N];
	int o;

	for (o = 0; o < oo[i]; o++) {
		int j = oj[i][o];

		dfs(j);
	}
	for (o = 0; o < oo[i]; o++) {
		int j = oj[i][o];

		used[aa[j]] = 1;
	}
	while (used[aa[i]])
		aa[i]++;
	for (o = 0; o < oo[i]; o++) {
		int j = oj[i][o];

		used[aa[j]] = 0;
	}
}

int main() {
	static int pp[N];
	static char visited[N];
	int n, i, k, a;

	scanf("%d", &n);
	for (i = 0; i < n; i++)
		scanf("%d", &pp[i]), pp[i]--;
	i = 0;
	while (!visited[i])
		visited[i] = 1, i = pp[i];
	k = 0;
	while (visited[i] == 1)
		visited[i] = 2, i = pp[i], k++;
	if (k % 2 == 0) {
		printf("POSSIBLE\n");
		return 0;
	}
	for (i = 0; i < n; i++)
		oj[i] = (int *) malloc(2 * sizeof *oj[i]);
	for (i = 0; i < n; i++)
		if (visited[i] != 2)
			link(pp[i], i);
	a = -1;
	for (i = 0; i < n; i++)
		if (visited[i] == 2) {
			dfs(i);
			if (a != -1 && a != aa[i]) {
				printf("POSSIBLE\n");
				return 0;
			}
			a = aa[i];
		}
	printf("IMPOSSIBLE\n");
	return 0;
}
