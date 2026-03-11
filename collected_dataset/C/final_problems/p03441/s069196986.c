#include <stdio.h>

typedef struct List {
	struct List *next;
	int v;
} list;

int main()
{
	int i, N, u, w, deg[100001] = {};
	list *adj[100001] = {}, e[200001], *p;
	scanf("%d", &N);
	if (N == 2) {
		printf("1\n");
		fflush(stdout);
		return 0;
	}
	for (i = 0; i < N - 1; i++) {
		scanf("%d %d", &u, &w);
		e[i*2].v = w + 1;
		e[i*2+1].v = u + 1;
		e[i*2].next = adj[u+1];
		e[i*2+1].next = adj[w+1];
		adj[u+1] = &(e[i*2]);
		adj[w+1] = &(e[i*2+1]);
		deg[u+1]++;
		deg[w+1]++;
	}
	
	int par[100001] = {}, q[100001], head, tail;
	for (i = 1; i <= N; i++) if (deg[i] > 1) break;
	par[i] = i;
	q[0] = i;
	for (head = 0, tail = 1; head < tail; head++) {
		u = q[head];
		for (p = adj[u]; p != NULL; p = p->next) {
			w = p->v;
			if (par[w] == 0) {
				par[w] = u;
				q[tail++] = w;
			}
		}
	}
	
	int flag[100001] = {}, ans = 0, count;
	for (head--; head >= 0; head--) {
		u = q[head];
		for (p = adj[u], count = 0; p != NULL; p = p->next) {
			if (p->v == par[u]) continue;
			else if (flag[p->v] == 0) count++;
			else flag[u] = 1;
		}
		if (count >= 2) {
			ans += count - 1;
			flag[u] = 1;
		}
	}
	printf("%d\n", ans);
	fflush(stdout);
	return 0;
}