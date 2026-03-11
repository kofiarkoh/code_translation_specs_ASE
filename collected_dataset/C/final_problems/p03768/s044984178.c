#include <stdio.h>
#include <stdlib.h>

typedef struct List{
	struct List* next;
	int value;
} list;

int N, M, Q;
int color[100001][2];
list edge[100001];

int max(int a, int b) { return a > b ? a : b; }

void make_edge(int n1, int n2) {
	list *node, *p = &edge[n1];
	while (p->next != NULL) p = p->next;
	node = malloc(sizeof(list));
	node->next = NULL;
	node->value = n2;
	p->next = node;
}

void print_edge() {
	int i;
	list *p;
	for (i = 1; i <= N; i++) {
		printf("%d:", i);
		p = &edge[i];
		while (p != NULL) {
			printf("%d ", p->value);
			p = p->next;
		}
		printf("\n");
	}
}

void dfs(int v, int d, int c) {
	if (color[v][0] == c && color[v][1] > d) return;
	color[v][0] = c;
	color[v][1] = max(color[v][1], d);
	if (d <= 0) return;

	list *p = edge[v].next;
	while (p != NULL) {
		dfs(p->value, d - 1, c);
		p = p->next;
	}
}

int main(void) {
	int i, j, a, b, v, d, c;

	scanf("%d%d", &N, &M);
	for (i = 1; i <= N; i++) {
		edge[i].next = NULL;
		edge[i].value = i;
	}
	for (i = 0; i < M; i++) {
		scanf("%d%d", &a, &b);
		make_edge(a, b);
		make_edge(b, a);
	}

	scanf("%d", &Q);
	for (i = 0; i < Q; i++) {
		scanf("%d%d%d", &v, &d, &c);
		for (j = 1; j <= N; j++) color[j][1] = -1;
		dfs(v, d, c);
	}
	
	for (i = 1; i <= N; i++) {
		printf("%d\n", color[i][0]);
	}
	return 0;
}