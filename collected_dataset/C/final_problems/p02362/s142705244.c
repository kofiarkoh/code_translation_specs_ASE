// AOJ GRL_1_B Single Source Shortest Path (Negative Edges)
// 2018.5.4 bal4u

#include <stdio.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	if (c == '-') {	c = gc();
		do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
		return -n;
	}
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

#define INF   0x10101010
#define MAX_V  1002
#define MAX_E  2002

typedef struct { int s, t, d; } EDGE;
EDGE edge[MAX_E]; int E;
int V;
int dist[MAX_V];

int bellmanFord(int s)
{
	int i;
	int update, cnt;
	EDGE *ep;

	memset(dist, INF, V<<2);
	dist[s] = 0;	// start node
	update = 1, cnt = 0;
	while (update) {
		update = 0;
		if (++cnt > V) return -1;
		for (i = 0; i < E; i++) {
			ep = edge+i;
			if (dist[ep->s] != INF && dist[ep->t] > dist[ep->s] + ep->d) {
				dist[ep->t] = dist[ep->s] + ep->d;
				update = 1;
			}
		}
	}
	return 0;
}

int main()
{
	int s, i;

	V = in(), E = in(), s = in();
	for (i = 0; i < E; i++) {
		edge[i].s = in(), edge[i].t = in(), edge[i].d = in();
	}
	if (bellmanFord(s) < 0) puts("NEGATIVE CYCLE");
	else for (i = 0; i < V; i++) {
		if (dist[i] == INF) puts("INF");
		else printf("%d\n", dist[i]);
	}
	return 0;
}

