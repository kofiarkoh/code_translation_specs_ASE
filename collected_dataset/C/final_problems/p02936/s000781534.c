#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int parent;
	long long total;
	int visit;
}node_t;

node_t node[200030] = { 0 };

int root(int node_n) {
	if (node_n == 1) return 1;
	else if (node_n == 0)return 0;
	else 
	return root(node[node_n].parent);
}
void joint(int upper, int lower){
	int temp = node[lower].parent;

	node[lower].parent = upper;
	if (temp != 0)
		joint(lower, temp);
}
long long ans(int i) {
	if (node[i].visit)return node[i].total;
	else {
		node[i].visit = 1;
		node[i].total += ans(node[i].parent);
		return node[i].total;
	}
}
int main(void) {
	int n, q;
	int a, b;
	int p, x;

	scanf("%d %d", &n, &q);
	node[1].parent = 1;
	node[1].visit = 1;
	for (int i = 0; i < n - 1; i++) {
		scanf("%d %d", &a, &b);
		if (a != 1 && root(b)) joint(b, a);
		else joint(a, b);
	}
	for (int i = 0; i < q; i++) {
		scanf("%d %d", &p, &x);
		node[p].total += x;
	}
	for (int i = 1; i <= n; i++) {
		printf("%lld ", ans(i));
	}
}