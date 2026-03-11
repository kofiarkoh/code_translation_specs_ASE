// AOJ 	2168 Luigi's Tavern
// 2018.3.12 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*******************************************/
/* Dinic's Max Flow Algorithm 
/*******************************************/

#define INF 0x01010101
#define VMAX 410
typedef struct { int to; int cap; int rev; } EDGE;
EDGE edge[VMAX][VMAX]; int hi[VMAX];
int V;				// 頂点数
int level[VMAX], iter[VMAX];
int queue[2*VMAX], qtop;

void add_edge(int from, int to, int cap)
{
	int f, t;
	EDGE *e;

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

/**********************************************/

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0, c = getchar_unlocked();
	if (c == '-') {	c = getchar_unlocked();
		do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
		return -n;
	}
	do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
	return n;
}

int main()
{
	int H, W, C, M, NW, NC, NM, i, k;
	int n, nw, nc, nm, N, source, sink;

	while ((H = in()) >= 0) {
		W = in(), C = in(), M = in();
		NW = in(), NC = in(), NM = in();

		n = H+W+C+M, nw = n, nc = n+1, nm = n+2, N = n+3;
		source= 2*N, sink = source+1, V = sink+1;
		memset(hi, 0, V << 2);

		for (i = 0; i < H; i++) add_edge(source, N+i, 1);
		for (i = 0; i < n; i++) add_edge(N+i, i, 1);
		add_edge(N+nw, nw, NW), add_edge(N+nc, nc, NC), add_edge(N+nm, nm, NM);
		for (i = 0; i < H; i++) add_edge(i, N+nw, 1);
		for (i = 0; i < W; i++) {
			k = in(); while (k--) add_edge(in()-1, N+H+i, 1);
			add_edge(H+i, N+nc, 1);
		}
		for (i = 0; i < C; i++) {
			k = in(); while (k--) add_edge(in()-1+H, N+H+W+i, 1);
			add_edge(H+W+i, N+nm, 1);
		}
		for (i = 0; i < M; i++) {
			k = in(); while (k--) add_edge(in()-1+H+W, N+H+W+C+i, 1);
			add_edge(H+W+C+i, sink, 1);
		}
		for (i = 0; i < C; i++) add_edge(nw, N+H+W+i, 1);
		for (i = 0; i < M; i++) add_edge(nc, N+H+W+C+i, 1);
		add_edge(nm, sink, NM);

		printf("%d\n", maxFlow(source, sink));
	}
	return 0;
}
