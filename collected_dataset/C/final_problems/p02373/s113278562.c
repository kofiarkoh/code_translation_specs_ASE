#include <stdio.h>
#include <stdlib.h>

#define MAX_NODE (100000)
#define MAX_DEPTH (16)

int parent[MAX_NODE] = {0};
int so_list[MAX_NODE];
int si_list[MAX_NODE];
int so;
int si;
int tree_cache[MAX_NODE][MAX_DEPTH];
int t_size[MAX_NODE];

void init_cache(int root, int n) {
	int i,j,node;
	for(i=0;i<n;i++) {
		tree_cache[i][0] = i;
		for(j=1;j<MAX_DEPTH;j++) {
			node = parent[tree_cache[i][j-1]];
			tree_cache[i][j] = node;
			if(node == root) {
				t_size[i] = j + 1;
				break;
			}
		}
	}
}

int binary_search(int left, int right, int* so, int* si, int root) {
	int i, node;
	int middle = (left + right) / 2;
	if(right - left < 40) {
		for(i=left,node=root;i<=right;i++) {
			if(so[i] == si[i]) {
				return so[i];
			}
		}
		return node;
	}
	
	if(so[middle] != si[middle]) {
		return binary_search(middle, right, so, si, root);
	}
	
	return binary_search(left, middle, so, si, root);
}

int solve(int source, int sink, int root) {
	int node;
	int last;
	int* p_so_list = so_list;
	int* p_si_list = si_list;
	if(source == sink) return source;
	so = si = 0;

	// sourceからrootへ行く際に通るnodeを登録
	if(t_size[source] == -1) {
		node = source;
		while(node != root) {
			so_list[so++] = node;
			node = parent[node];
		}
		so_list[so++] = node;
	} else {
		p_so_list = tree_cache[source];
		so = t_size[source];
	}

	// sinkからrootへ行く際に通るnodeを登録
	if(t_size[sink] == -1) {
		node = sink;
		while(node != root) {
			si_list[si++] = node;
			node = parent[node];
		}
		si_list[si++] = node;
	} else {
		p_si_list = tree_cache[sink];
		si = t_size[sink];
	}

	if(si > so) {
		last = si - so;
		return binary_search(0, so, p_so_list, &p_si_list[last], root);
	}
	
	last = so - si;
	return binary_search(0, si, &p_so_list[last], p_si_list, root);
	
}

int main(void) {
	int i, j, m, n, t, q;
	int source, sink;
	int is_root[MAX_NODE] = {0};
	int root;
	
	scanf("%d\n", &n);
	for(i=0;i<n;i++) {
		scanf("%d", &m);

		for(j=0;j<m;j++) {
			scanf("%d", &t);
			is_root[t] = -1;
			parent[t] = i;
		}
	}
	
	// ルート
	root = 0;
	for(i=0;i<n;i++) {
		t_size[i] = -1;
		if(is_root[i] == 0) {
			root = i;
			break;
		}
	}
	
	// 浅いものはあらかじめ作っておく
	init_cache(root, n);
	
	scanf("%d\n", &q);
	for(i=0;i<q;i++) {
		scanf("%d %d\n" ,&source, &sink);
		printf("%d\n", solve(source, sink, root));
	}

	return 0;
}