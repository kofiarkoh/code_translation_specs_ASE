#include <stdio.h>
#include <string.h>

int data[10000];
int init(void){ memset(data, -1, sizeof(data)); }
int root(int x){ return data[x] < 0 ? x : (data[x] = root(data[x])); }
int size(int x){ return -data[root(x)]; }
int find(int x, int y){ return root(x) == root(y); }
int merge(int x, int y){
	x = root(x);
	y = root(y);
	if (x != y){
		if (data[y] < data[x]) x ^= y ^= x ^= y;
		data[x] += data[y];
		data[y] = x;
	}
	return x != y;
}

typedef struct { int from, to; int v; } Edge;
int cmp_edge(Edge *a, Edge *b){ return a->v - b->v; }
Edge V[10000];

int main(void)
{
	int n;

	while (scanf("%d", &n), n){
		int m, i;
		int minv;
		int count;

		init();

		scanf("%d", &m);
		for (i = 0; i < m; i++){
			int from, to, v;
			scanf("%d,%d,%d", &from, &to, &v);
			V[i] = (Edge){from, to, v / 100 - 1};
		}
		
		qsort(V, m, sizeof(Edge), cmp_edge);

		minv = count = 0;
		for (i = 0; i < m; i++){
			if (count >= n) break;
			if (!find(V[i].from, V[i].to)){
				if (merge(V[i].from, V[i].to)) minv += V[i].v, count++;
			}
		}
		printf("%d\n", minv);
	}
	return (0);
}