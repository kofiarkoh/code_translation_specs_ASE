// AOJ 2290 Attack the Moles
// 2018.3.14 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INF 0x01010101
#define MAX 6005
typedef struct { int to, rev; int cap, cost; } EDGE;
EDGE *edge[MAX]; int hi[MAX], lim[MAX];
int V;				// 頂点数
int dist[MAX], prevv[MAX], preve[MAX];
int h[MAX];

void check_lim(int a)
{
	if (hi[a] < lim[a]) return;
	if (lim[a] == 0) {
		lim[a] = 32, hi[a] = 0;
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
					nt = dist[v] + e->cost + h[v]-h[e->to];
					if (dist[nv] > nt) {
						dist[nv] = nt, prevv[nv] = v, preve[e->to] = i;
						update = 1;
					}
				}
			}
		} while (update);

		if (dist[T] == INF) return -1;
		for (v = 0; v < V; v++) h[v] += dist[v];

		t = F;
		for (v = T; v != S; v = prevv[v]) {
			e = &edge[prevv[v]][preve[v]];
			if (t > e->cap) t = e->cap;
		}
		F -= t;

		ret += t * h[T];
		for (v = T; v != S; v = prevv[v]) {
			e = &edge[prevv[v]][preve[v]];
			e->cap -= t;
			edge[v][e->rev].cap += t;
		}
	}
	return ret;
}

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

#define ABS(a)     ((a)>=0?(a):-(a))

int x[3003], t[3003], p[3003]; 

// for topologicalSort
int  Q[MAX+5], top, end;
int  count[MAX];
void topologicalSort(int source)
{
	int i, j;
	EDGE *e;

	top = end = 0;
	for (i = 0; i < V; i++) for (j = 0; j < hi[i]; j++) if (edge[i][j].cap) count[edge[i][j].to]++;
	memset(h, INF, V<<2);
	for (i = 0; i < V; i++) if (!count[i]) { Q[end++] = i; if (i == source) h[i] = 0; }

	while (top < end) {
		i = Q[top++];
		for (j = 0; j < hi[i]; j++) {
			e = &edge[i][j];
			if (e->cap) {
				if (h[e->to] > h[i] + e->cost) h[e->to] = h[i] + e->cost;
				if (--count[e->to] == 0) Q[end++] = e->to;
			}
		}
	}
}

int main()
{
	int n, v, xl, xr, i, j;
	int source, sink, a, b;

	n = in(), v = in(), xl = in(), xr = in();
	for (i = 0; i < n; i++) x[i] = in(), t[i] = in(), p[i] = in();

	a = 2*n, b = a+1, source = b+1, sink = source+1, V = sink+1;

	add_edge(source, sink, 2, 0);
	add_edge(source, a, 1, 0), add_edge(source, b, 1, 0);
	for (i = 0; i < n; i++) {
		if (ABS(x[i]-xl) <= t[i]*v) add_edge(a, i<<1, 1, 0);
		if (ABS(x[i]-xr) <= t[i]*v) add_edge(b, i<<1, 1, 0);
		add_edge((i<<1)+1, sink, 1, 0);
		add_edge(i<<1, (i<<1)+1, 1, -p[i]);
		for (j = 0; j < n; j++) if (i != j) {
			if ((t[j]-t[i])*v >= ABS(x[i]-x[j])) add_edge((i<<1)+1, j<<1, 1, 0);
		}
	}
	topologicalSort(source);
	printf("%d\n", -minCostFlow(source, sink, 2));
	return 0;
}
