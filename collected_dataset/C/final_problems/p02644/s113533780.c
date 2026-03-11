#include <stdio.h>
#include <stdlib.h>

int main()
{
	int i, j, H, W, K, x[2], y[2];
	char **c;
	scanf("%d %d %d", &H, &W, &K);
	scanf("%d %d %d %d", &(x[0]), &(y[0]), &(x[1]), &(y[1]));
	c = (char**)malloc(sizeof(char*) * (H + 2));
	for (i = 1; i <= H; i++) {
		c[i] = (char*)malloc(sizeof(char) * (W + 2));
		scanf("%s", &(c[i][1]));
	}
	c[0] = (char*)malloc(sizeof(char) * (W + 2));
	c[H+1] = (char*)malloc(sizeof(char) * (W + 2));
	for (i = 1; i <= H; i++) {
		c[i][0] = '@';
		c[i][W+1] = '@';
	}
	for (j = 1; j <= W; j++) {
		c[0][j] = '@';
		c[H+1][j] = '@';
	}
	
	int k, l, q[1000001][2], head, tail;
	int **dist = (int**)malloc(sizeof(int*) * (H + 2));
	for (i = 1; i <= H; i++) {
		dist[i] = (int*)malloc(sizeof(int) * (W + 2));
		for (j = 1; j <= W; j++) dist[i][j] = 1 << 30;
	}
	dist[x[0]][y[0]] = 0;
	q[0][0] = x[0];
	q[0][1] = y[0];
	for (head = 0, tail = 1; head < tail; head++) {
		i = q[head][0];
		j = q[head][1];
		for (k = i - 1; k >= i - K; k--) {
			if (c[k][j] == '@' || dist[k][j] <= dist[i][j]) break;
			dist[k][j] = dist[i][j] + 1;
			q[tail][0] = k;
			q[tail++][1] = j;
		}
		for (k = i + 1; k <= i + K; k++) {
			if (c[k][j] == '@' || dist[k][j] <= dist[i][j]) break;
			dist[k][j] = dist[i][j] + 1;
			q[tail][0] = k;
			q[tail++][1] = j;
		}
		for (l = j - 1; l >= j - K; l--) {
			if (c[i][l] == '@' || dist[i][l] <= dist[i][j]) break;
			dist[i][l] = dist[i][j] + 1;
			q[tail][0] = i;
			q[tail++][1] = l;
		}
		for (l = j + 1; l <= j + K; l++) {
			if (c[i][l] == '@' || dist[i][l] <= dist[i][j]) break;
			dist[i][l] = dist[i][j] + 1;
			q[tail][0] = i;
			q[tail++][1] = l;
		}
	}
	if (dist[x[1]][y[1]] < 1 << 30) printf("%d\n", dist[x[1]][y[1]]);
	else printf("-1\n");
	fflush(stdout);
	return 0;
}