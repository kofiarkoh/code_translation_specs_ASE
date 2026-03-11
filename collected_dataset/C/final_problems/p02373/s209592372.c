#include <stdio.h>
#include <stdlib.h>

#define MAX_NODE (100000)

int* node[MAX_NODE];
int n_size[MAX_NODE] = {0};
int parent[MAX_NODE] = {0};
int so_list[MAX_NODE];
int si_list[MAX_NODE];
int so;
int si;

int solve(int source, int sink, int root) {
	int node;
	if(source == sink) return source;
	so = si = 0;
	
	// sourceからrootへ行く際に通るnodeを登録
	node = source;
	while(node != root) {
		so_list[so++] = node;
		node = parent[node];
	}
	
	// sinkからrootへ行く際に通るnodeを登録
	node = sink;
	while(node != root) {
		si_list[si++] = node;
		node = parent[node];
	}
	
	// ルートから順番に追ってみて、同じ親じゃなくなったら
	// ひとつ前のノードを結果とする
	node = root;
	for(;--si && --so;) {
		if(so_list[so] != si_list[si]) {
			return node;
		}
		node = so_list[so];
	}
	
	return node;
	
}

int main(void) {
	int i, j, x, m, n, q;
	int source, sink;
	int is_root[MAX_NODE] = {0};
	int root;
	
	scanf("%d\n", &n);
	for(i=0;i<n;i++) {
		scanf("%d", &m);
		n_size[i] = m;
		if(m>0) node[i] = (int*)malloc(m*sizeof(int));
		
		for(j=0;j<m;j++) {
			scanf("%d", &node[i][j]);
			is_root[node[i][j]] = -1;
			parent[node[i][j]] = i;
		}
	}
	
	// ルート
	root = 0;
	for(i=0;i<n;i++) {
		if(is_root[i] == 0) {
			root = i;
			break;
		}
	}
	
	scanf("%d\n", &q);
	for(i=0;i<q;i++) {
		scanf("%d %d\n" ,&source, &sink);
		printf("%d\n", solve(source, sink, root));
	}

	return 0;
}