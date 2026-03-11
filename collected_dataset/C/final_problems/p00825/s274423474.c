// AOJ 1246: Concert Hall Scheduling
// 2017.11.8 bal4u@uu
// 2018.4.21
 
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
 
#define INF 0x10101010
#define MAX 400
typedef struct { int to, rev; int cap, cost; } EDGE;
EDGE *edge[MAX]; int hi[MAX], lim[MAX];
int  V;             // 頂点数
int  dist[MAX], prevv[MAX], preve[MAX];

void check_lim(int a)
{
	if (hi[a] < lim[a]) return;
	if (lim[a] == 0) {
		lim[a] = 16, hi[a] = 0;
		edge[a] = malloc(sizeof(EDGE) *lim[a]);
	} else if (hi[a] == lim[a]) {
		int k = lim[a];
		if (k > 512) k = 512;
		lim[a] += k;
		edge[a] = realloc(edge[a], sizeof(EDGE) *lim[a]);
	}
}

void add_edge(int from, int to, int cap, int cost)
{
	int f, t;
	EDGE *e;

	check_lim(from), check_lim(to);
	f = hi[from]++, t = hi[to]++;
	e = &edge[from][f], e->to = to, e->cap = cap, e->cost = cost, e->rev = t; 
	e = &edge[to][t], e->to = from, e->cap = 0, e->cost = -cost, e->rev = f;
}

int minCostFlow(int S, int T, int F)
{
	int i, v, nv, t, nt, update, ret;
	EDGE *e;

	ret = 0;
  
	while (F > 0) {
		memset(dist, INF, V << 2);
		dist[S] = 0;
		do {
			update = 0;
			for (v = 0; v < V; v++) {
				if (dist[v] == INF) continue;
				for (i = 0; i < hi[v]; i++) {
					e = &edge[v][i];
					if (e->cap <= 0) continue;
					nv = e->to;
					nt = dist[v] + e->cost;
					if (dist[nv] > nt) {
						dist[nv] = nt, prevv[nv] = v, preve[e->to] = i;
						update = 1;
					}
				}
			}
		} while (update);

		if (dist[T] == INF) return -1;

		t = F;
		for (v = T; v != S; v = prevv[v]) {
			e = &edge[prevv[v]][preve[v]];
			if (t > e->cap) t = e->cap;
		}
		F -= t;

		ret += t * dist[T];
		for (v = T; v != S; v = prevv[v]) {
			e = &edge[prevv[v]][preve[v]];
			e->cap -= t;
			edge[v][e->rev].cap += t;
		}
	}
	return ret;
}
 
// バッファを経ずstdinから数値を得る
//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

int main()
{
	int n, i, j, k, w;
	int source, sink;

	while (n = in()) {
		source = 0, sink = 365;
		V = sink+1;
		memset(hi, 0, V<<2), memset(lim, 0, V<<2);
		for (k = 0; k < n; k++) {
			i = in(), j = in(), w = in();
			add_edge(i-1, j, 1, -w);
		}
		for (i = 0; i < 365; i++) add_edge(i, i+1, 2, 0);
		printf("%d\n", -minCostFlow(source, sink, 2));
	}
	return 0;
}
