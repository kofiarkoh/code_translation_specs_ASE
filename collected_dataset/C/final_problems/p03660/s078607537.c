#include <stdio.h>
#include <stdlib.h>

typedef struct List {
	struct List *next;
	int v;
} list;

typedef struct {
	list **adj;
} graph;

int main()
{
	int i, u, w, N;
	scanf("%d", &N);
	list **adj = (list**)malloc(sizeof(list*) * (N + 1)), *d = (list*)malloc(sizeof(list) * N * 2);
	for (i = 0; i < N - 1; i++) {
		scanf("%d %d", &u, &w);
		d[i*2].v = w;
		d[i*2+1].v = u;
		d[i*2].next = adj[u];
		d[i*2+1].next = adj[w];
		adj[u] = &(d[i*2]);
		adj[w] = &(d[i*2+1]);
	}
	
	list *p;
	int flag[100001] = {}, q[100001], head, tail;
	flag[1] = 1;
	flag[N] = -1;
	q[0] = 1;
	q[1] = N;
	for (head = 0, tail = 2; head < tail; head++) {
		for (p = adj[q[head]]; p != NULL; p = p->next) {
			if (flag[p->v] == 0) {
				flag[p->v] = flag[q[head]];
				q[tail++] = p->v;
			}
		}
	}
	
	int count = 0;
	for (i = 1; i <= N; i++) count += flag[i];
	if (count > 0) printf("Fennec\n");
	else printf("Snuke\n");
	fflush(stdout);
	return 0;
}