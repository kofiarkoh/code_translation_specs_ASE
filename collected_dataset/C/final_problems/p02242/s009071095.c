#include <stdio.h>
#define INF 10000000
#define WHITE 0
#define GRAY 1
#define BLACK 2

int M[100][100];

void dijkstra(int n) {
	int i, u, minv, d[100], color[100];

	for (i = 0; i < n; i++) {
		d[i] = INF;
		color[i] = WHITE;
	}
	d[0] = 0;
	while (1) {
		minv = INF;
		u = -1;
		for (i = 0; i < n; i++) {
			if (minv > d[i] && color[i] != BLACK) {
				u = i;
				minv = d[i];
			}
		}
		if (u == -1)break;
		color[u] = BLACK;
		for (i = 0; i < n; i++) {
			if (color[i] != BLACK && M[u][i] != INF) {
				if (d[i] > d[u] + M[u][i]) {
					d[i] = d[u] + M[u][i];
					color[i] = GRAY;
				}
			}
		}
	}
	for (i = 0; i < n; i++) {
		printf("%d ", i);
		if (d[i] == INF) {
			printf("%d\n", -1);
		}
		else {
			printf("%d\n", d[i]);
		}
	}
}

int main() {
	int n, i, j, u, k, c, v;
	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		for (j = 0; j < n; j++) M[i][j] = INF;
	}

	for (i = 0; i < n; i++) {
		scanf("%d %d", &u, &k);
		for (j = 0; j < k; j++) {
			scanf("%d %d", &v, &c);
			M[u][v] = c;
		}
	}
	dijkstra(n);
}
