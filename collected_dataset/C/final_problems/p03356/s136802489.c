#include <stdio.h>
#include <stdlib.h>


typedef struct List {
	struct List *next;
	int v;
} list;

int main()
{
	int i, N, M, p[100001], x[100001], y[100001];
	scanf("%d %d", &N, &M);
	for (i = 1; i <= N; i++) scanf("%d", &(p[i]));
	for (i = 1; i <= M; i++) scanf("%d %d", &(x[i]), &(y[i]));

	list **adj = (list**)malloc(sizeof(list*) * (N + 1)), *d = (list*)malloc(sizeof(list) * M * 2);
	for (i = 0; i < M; i++) {
		d[i*2].v = y[i+1];
		d[i*2+1].v = x[i+1];
		d[i*2].next = adj[x[i+1]];
		d[i*2+1].next = adj[y[i+1]];
		adj[x[i+1]] = &(d[i*2]);
		adj[y[i+1]] = &(d[i*2+1]);
	}
	
	list *po;
	int k, u, w, comp[100001] = {}, q[100001], head, tail;
	for (i = 1, k = 1; i <= N; i++) {
		if (comp[i] != 0) continue;
		comp[i] = k;
		q[0] = i;
		for (head = 0, tail = 1; head < tail; head++) {
			u = q[head];
			for (po = adj[u]; po != NULL; po = po->next) {
				w = po->v;
				if (comp[w] == 0) {
					comp[w] = comp[u];
					q[tail++] = w;
				}
			}
		}
		k++;
	}
	
	int ans = 0;
	for (i = 1; i <= N; i++)if (comp[i] == comp[p[i]]) ans++;
	printf("%d\n", ans);
	fflush(stdout);
	return 0;
}