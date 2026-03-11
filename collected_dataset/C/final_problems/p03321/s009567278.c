#include <stdio.h>
#include <stdlib.h>

typedef struct List {
	struct List *next;
	int v;
} list;

int DFS(list** adj, int s, int flag[])
{
	int size = flag[s];
	list *p;
	for (p = adj[s]; p != NULL; p = p->next) {
		if (flag[p->v] == 0) {
			flag[p->v] = -flag[s];
			size += DFS(adj, p->v, flag);
		} else if (flag[p->v] == flag[s]) return -100000;
	}
	return size;
}

int main()
{
	int i, u, w, N, M, mat[701][701] = {};
	scanf("%d %d", &N, &M);
	for (i = 1; i <= M; i++) {
		scanf("%d %d", &u, &w);
		mat[u][w] = 1;
		mat[w][u] = 1;
	}
	
	list **adj = (list**)malloc(sizeof(list*) * (N + 1)), *d = (list*)malloc(sizeof(list) * (N * (N - 1) - M * 2));
	for (u = 1, i = 0; u < N; u++) {
		for (w = u + 1; w <= N; w++) {
			if (mat[u][w] == 0) {
				d[i*2].v = w;
				d[i*2+1].v = u;
				d[i*2].next = adj[u];
				d[i*2+1].next = adj[w];
				adj[u] = &(d[i*2]);
				adj[w] = &(d[i*2+1]);
				i++;
			}
		}
	}
	
	int k, flag[701] = {}, size[701];
	for (i = 1, k = 0; i <= N; i++) {
		if (flag[i] != 0) continue;
		flag[i] = 1;
		size[++k] = DFS(adj, i, flag);
		if (size[k] < -1000) break;
		else size[k] = abs(size[k]);
	}
	if (i <= N) {
		printf("-1\n");
		fflush(stdout);
		return 0;
	}
	
	int j, l[2], dp[2][1401] = {}, pos[2][701];
	for (i = 1, l[0] = 1, dp[0][N] = 1, pos[0][0] = N; i <= k; i++) {
		l[i%2] = 0;
		for (j = 0; j < l[1-i%2]; j++) {
			if (pos[1-i%2][j] >= size[i] && dp[i%2][pos[1-i%2][j] - size[i]] == 0) {
				dp[i%2][pos[1-i%2][j] - size[i]] = 1;
				pos[i%2][l[i%2]++] = pos[1-i%2][j] - size[i];
			}
			if (pos[1-i%2][j] <= N * 2 - size[i] && dp[i%2][pos[1-i%2][j] + size[i]] == 0) {
				dp[i%2][pos[1-i%2][j] + size[i]] = 1;
				pos[i%2][l[i%2]++] = pos[1-i%2][j] + size[i];
			}
			dp[1-i%2][pos[1-i%2][j]] = 0;
		}
	}
	
	for (i = N; i >= 0; i--) if (dp[k%2][i] == 1) break;
	for (j = N; j <= N * 2; j++) if (dp[k%2][j] == 1) break;
	j = (N - i > j - N)? j - N: N - i;
	printf("%d\n", ((N + j) * (N + j - 2) + (N - j) * (N - j - 2)) / 8);
	fflush(stdout);
	return 0; 
}