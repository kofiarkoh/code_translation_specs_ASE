// AOJ 1615 Gift Exchange Party
// 2018.3.31 bal4u
 
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
 
/*******************************************/
/* Dinic's Max Flow Algorithm 
/*******************************************/

#define INF 0x01010101
#define VMAX 5100
typedef struct { int to; int cap; int rev; } EDGE;
EDGE *edge[VMAX]; int hi[VMAX], lim[VMAX];
int V;				// 頂点数
int level[VMAX], iter[VMAX];
int queue[2*VMAX], qtop;

void check_lim(int a)
{
	if (hi[a] < lim[a]) return;
	if (lim[a] == 0) {
		lim[a] = 32, hi[a] = 0;
		edge[a] = malloc(sizeof(EDGE) *lim[a]);
	} else if (hi[a] == lim[a]) {
		int k = lim[a];
		if (k > 1024) k = 1024;
		lim[a] += k;
		edge[a] = realloc(edge[a], sizeof(EDGE) *lim[a]);
	}
}

void add_edge(int from, int to, int cap)
{
	int f, t;
	EDGE *e;

	check_lim(from), check_lim(to);
	f = hi[from]++, t = hi[to]++;
	e = &edge[from][f], e->to = to, e->cap = cap, e->rev = t; 
	e = &edge[to][t], e->to = from, e->cap = 0, e->rev = f;
}

void bfs(int s)
{
	int i, v;
	EDGE *e;
	
	memset(level, -1, V << 2);
	level[s] = 0;
	queue[0] = s, qtop =1;
	while (qtop) {
		v = queue[--qtop];
		for (i = 0; i < hi[v]; i++) {
			e = &edge[v][i];
			if (e->cap > 0 && level[e->to] < 0) {
				level[e->to] = level[v] + 1;
				queue[qtop++] = e->to;
			}
		}
	}
}
 
int dfs(int v, int t, int f)
{
	int *i, d;
	EDGE *e;
	
	if (v == t) return f;
	for (i = &iter[v]; *i < hi[v]; (*i)++) {
		e = &edge[v][*i];
		if (e->cap > 0 && level[v] < level[e->to]) {
			d = dfs(e->to, t, f <= e->cap? f: e->cap);
			if (d > 0) {
				e->cap -= d;
				edge[e->to][e->rev].cap += d;
				return d;
			}
		}
	}
	return 0;
}
 
int maxFlow(int s, int t)
{
	int f, flow = 0;
	while (1) {
		bfs(s);
		if (level[t] < 0) break;
		memset(iter, 0, V << 2);
		while ((f = dfs(s, t, INF)) > 0) flow += f;
	}
	return flow;
}

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

int n, m;
int u[5000], v[5000];

int himax;

int gmin()
{
	int i, j, f, ans;
	int source, sink;

	source = n+m, sink = source+1, V = sink+1;
	memset(lim, 0, V << 2);
	memset(hi, 0, V << 2);
	for (i = 0; i < n; i++) add_edge(source, i, 0);
	for (i = 0; i < m; i++) {
		add_edge(u[i], n+i, 1);
		add_edge(v[i], n+i, 1);
		add_edge(n+i, sink, 1);
    }

	f = 0, ans = 0;
	for (j = 1; j < n; j++) {
		for (i = 0; i < hi[source]; i++) edge[source][i].cap++;
		f += maxFlow(source, sink);
		if (f == n*j) ans = j;
	}
	for (i = 0; i < V; i++) if (lim[i]) free(edge[i]);
	return ans;
}

int gmax()
{
	int i, f, ans;
	int source, sink;

	source = n+m, sink = source+1, V = sink+1;
	memset(lim, 0, V << 2);
	memset(hi, 0, V << 2);
	for (i = 0; i < n; i++) add_edge(source, i, 0);
	for (i = 0; i < m; i++) {
		add_edge(u[i], n+i, 1);
		add_edge(v[i], n+i, 1);
		add_edge(n+i, sink, 1);
    }

	f = 0, ans = 1;
	while (1) {
		for (i = 0; i < hi[source]; i++) edge[source][i].cap++;
		f += maxFlow(source, sink);
		if (f == m) break;
		ans++;
	}
	for (i = 0; i < V; i++) if (lim[i]) free(edge[i]);
	return ans;
}

int main()
{
	int i;

	while (n = in()) {
		m = in();
		for (i = 0; i < m; i++) u[i] = in()-1, v[i] = in()-1;
		printf("%d %d\n", gmin(), gmax());
	}
	return 0;
}
