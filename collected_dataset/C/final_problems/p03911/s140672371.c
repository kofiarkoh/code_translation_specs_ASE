#include <stdio.h>
#include <stdlib.h>

typedef struct List {
	struct List *next;
	int v;
} list;

int main()
{
	int i, j, N, M, K[100001], *L[100001];
	scanf("%d %d", &N, &M);
	for (i = 1; i <= N; i++) {
		scanf("%d", &(K[i]));
		L[i] = (int*)malloc(sizeof(int) * (K[i] + 1));
		for (j = 1; j <= K[i]; j++) scanf("%d", &(L[i][j]));
	}

	int m = 0;
	list **adj = (list**)malloc(sizeof(list*) * (N + M + 2)), *d = (list*)malloc(sizeof(list) * 200000);
	for (i = 1; i <= N + M; i++) adj[i] = NULL;
	for (i = 1; i <= N; i++) {
		for (j = 1; j <= K[i]; j++) {
			d[m*2].v = N + L[i][j];
			d[m*2+1].v = i;
			d[m*2].next = adj[i];
			d[m*2+1].next = adj[N+L[i][j]];
			adj[i] = &(d[m*2]);
			adj[N+L[i][j]] = &(d[m*2+1]);
			m++;
		}
	}
	
	int head, tail, q[200001], flag[200001] = {};
	list *p;
	q[0] = 1;
	flag[1] = 1;
	for (head = 0, tail = 1; head < tail; head++) {
		for (p = adj[q[head]]; p != NULL; p = p->next) {
			if (flag[p->v] == 0) {
				flag[p->v] = 1;
				q[tail++] = p->v;
			}
		}
	}
	
	for (i = 1; i <= N; i++) if (flag[i] == 0) break;
	if (i > N) printf("YES\n");
	else printf("NO\n");
	fflush(stdout);
	return 0;
}