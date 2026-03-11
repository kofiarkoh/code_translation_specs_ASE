// AOJ 3047 Shiritori
// 2019.8.13 bal4u

#include <stdio.h>
#include <string.h>

//// 入出力関係
#if 1
#define gc() getchar_unlocked()
#define pc(x) putchar_unlocked(x)
#else
#define gc() getchar()
#define pc(x) putchar(x)
#endif

int in() { // 非負整数の入力
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int ins(char *s) { // 文字列の入力　スペース以下の文字で入力終了
	char *p = s;
	do *s = gc();
	while (*s++ > ' ');
	*--s = 0;
	return s - p;
}

/*******************************************/
/* Dinic's Max Flow Algorithm 
/*******************************************/

#define INF 10005
#define VMAX 30
typedef struct { int to; int cap; int rev; } EDGE;
EDGE edge[VMAX][VMAX*2]; int hi[VMAX];
int level[VMAX], iter[VMAX];
int queue[VMAX], qtop;

void add_edge(int from, int to, int cap) {
	int f, t; EDGE *e;
	f = hi[from]++, t = hi[to]++;
	e = &edge[from][f], e->to = to, e->cap = cap, e->rev = t;
	e = &edge[to][t], e->to = from, e->cap = 0, e->rev = f;
}

void bfs(int s) {
	int i, v; EDGE *e;
	memset(level, -1, sizeof(level));
	level[s] = 0;
	queue[0] = s, qtop = 1;
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
 
int dfs(int v, int t, int f) {
	int *i, d; EDGE *e;
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
 
int maxFlow(int s, int t) {
	int f, flow = 0;
	while (1) {
		bfs(s);
		if (level[t] < 0) break;
		memset(iter, 0, sizeof(iter));
		while ((f = dfs(s, t, INF)) > 0) flow += f;
	}
	return flow;
}

//// 本問題関係
int rel[30][30];
char s[103];
char f[30];

int main()
{
	int i, j, k, N, S, T;

	N = in();
	while (N--) {
		int a, b, w = ins(s);
		a = s[0] - 'a', b = s[w-1] - 'a';
		if (a != b) rel[a][b]++;
		f[b] = 1;
	}
	T = 26;
	for (i = 0; i < 26; i++) if (f[i]) {
		int fi = 0, fo = 0;
		for (j = 0; j < 26; j++) fo += rel[i][j];
		S = i;
		memset(hi, 0, sizeof(hi));
		for (j = 0; j < 26; j++) for (k = 0; k < 26; k++) if (rel[j][k]) {
			if (k == i) add_edge(j, T, rel[j][k]);
			else        add_edge(j, k, rel[j][k]);
		}
		if (maxFlow(S, T) == fo) pc('a'+i), pc('\n');
	}
	return 0;
}

