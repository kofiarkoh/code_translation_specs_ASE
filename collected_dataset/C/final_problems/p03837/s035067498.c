#include <stdio.h>
#include <stdlib.h>

#define INF 114514810

struct iset_t {
	int data;
	struct iset_t* next[2];
};

struct iset_t* generateSet(int data, struct iset_t* next1, struct iset_t* next2) {
	struct iset_t* generated = malloc(sizeof(struct iset_t));
	generated->data = data;
	generated->next[0] = next1;
	generated->next[1] = next2;
	return generated;
}

int N, M;
int cost[128][128];
struct iset_t* usedEdges[128][128];

char edgeUsed[1024];

void traverseSet(struct iset_t* set) {
	if (set == NULL || set->data < 0) return;
	if (set->data > 0) edgeUsed[set->data] = 1;
	set->data = -1; /* visited */
	traverseSet(set->next[0]);
	traverseSet(set->next[1]);
}

int main(void) {
	int i, j, k;
	int count;
	if (scanf("%d%d", &N, &M) != 2) return 1;
	for (i = 1; i <= N; i++) {
		for (j = 1; j <= N; j++) {
			cost[i][j] = INF;
			usedEdges[i][j] = NULL;
		}
		cost[i][i] = 0;
	}
	for (i = 1; i <= M; i++) {
		int a, b, c;
		if (scanf("%d%d%d", &a, &b, &c) != 3) return 1;
		cost[a][b] = cost[b][a] = c;
		usedEdges[a][b] = usedEdges[b][a] = generateSet(i, NULL, NULL);
	}
	for (k = 1; k <= N; k++) {
		for (i = 1; i <= N; i++) {
			for (j = 1; j <= N; j++) {
				int current = cost[i][j];
				int candidate = cost[i][k] + cost[k][j];
				if (candidate < current) {
					cost[i][j] = candidate;
					usedEdges[i][j] = generateSet(0, usedEdges[i][k], usedEdges[k][j]);
				} else if (candidate == current) {
					usedEdges[i][j] = generateSet(0, usedEdges[i][j],
						generateSet(0, usedEdges[i][k], usedEdges[k][j]));
				}
			}
		}
	}
	for (i = 1; i <= N; i++) {
		for (j = 1; j <= N; j++) {
			traverseSet(usedEdges[i][j]);
		}
	}
	count = 0;
	for (i = 1; i <= M; i++) {
		if (!edgeUsed[i]) count++;
	}
	printf("%d\n", count);
	return 0;
}
