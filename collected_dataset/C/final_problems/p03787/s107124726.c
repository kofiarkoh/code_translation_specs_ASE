#include <stdio.h>
#include <stdlib.h>

typedef struct Tlist{
	struct Tlist *next;
	int num;
} list;

void make_node(int, int);
int dfs(int, int);

int N, M;
list edge[100010];
int used[100010];

int main(void) {
	int i, tmp[2], single = 0, bipartite = 0, other = 0, ans = 0;

	scanf("%d%d", &N, &M);
	// リストの初期化
	for (i = 1; i <= N; i++) {
		edge[i].next = NULL;
		edge[i].num = 0;
	}
	for (i = 0; i < M; i++) {
		scanf("%d%d", &tmp[0], &tmp[1]);
		make_node(tmp[0], tmp[1]);
		make_node(tmp[1], tmp[0]);
	}

	for (i = 1; i <= N; i++) {
		if (used[i] != 0) continue;

		if (edge[i].next == NULL)
			single++;
		else if (dfs(i, 1))
			bipartite++;
		else
			other++;
	}

	ans += single * (N - single) * 2 + single * single;
	ans += other * bipartite * 2 + other * other;
	ans += bipartite * bipartite * 2;
	printf("%d\n", ans);

	return 0;
}

void make_node(int index, int value) {
	list *node;

	node = &edge[index];
	while (node->next != NULL)
		node = node->next;
	
	node->next = (list *)malloc(sizeof(list));
	node = node->next;
	node->next = NULL;
	node->num = value;
}

// 二部グラフなら1、そうでなければ0を返す
int dfs(int v, int sign) {
	int res = 1;
	list *node;

	used[v] = sign;
	node = &edge[v];
	// usedを埋めるため途中で再帰を終了しない
	while (node->next != NULL) {
		node = node->next;
		if (used[node->num] == sign)
			res = 0;
		else if (used[node->num] == 0)
			res = dfs(node->num, -sign);
	}

	return res;
}