#include <stdio.h>
#include <stdlib.h>

#define MAX_NODE (100000)

int parent[MAX_NODE] = {0};
int so_list[MAX_NODE];
int si_list[MAX_NODE];
int so;
int si;

int binary_search(int left, int right, int* so, int* si) {
	int i, node;
	int middle = (left + right) / 2;
	if(right - left < 40) {
		for(i=left,node=so[left];i<=right;i++) {
			if(so[i] == si[i]) {
				return so[i];
			}
		}
		return node;
	}
	
	if(so[middle] != si[middle]) {
		return binary_search(middle, right, so, si);
	}
	
	return binary_search(left, middle, so, si);
}

int solve(int source, int sink, int root) {
	int node;
	int last;
	if(source == sink) return source;
	so = si = 0;

	// sourceからrootへ行く際に通るnodeを登録
	node = source;
	while(node != root) {
		so_list[so++] = node;
		node = parent[node];
	}
	si_list[so++] = node;

	// sinkからrootへ行く際に通るnodeを登録
	node = sink;
	while(node != root) {
		si_list[si++] = node;
		node = parent[node];
	}
	si_list[si++] = node;

	if(si > so) {
		last = si - so;
		return binary_search(0, so, so_list, &si_list[last]);
	}
	
	last = so - si;
	return binary_search(0, si, &so_list[last], si_list);
	
}

int main(void) {
	int i, j, x, m, n, t, q;
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