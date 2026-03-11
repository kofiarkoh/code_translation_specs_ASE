// AOJ 0351 Quiet Town
// 2018.2.25 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define QMAX 2000
typedef struct { int t, s; } QUE;
QUE que[QMAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;
	QUE qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].t < que[i].t) min = l; else min = i;
	if (r < qsize && que[r].t < que[min].t) min = r;
	if (min != i) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		min_heapify(min);
	}
}

void deq()
{
	que[0] = que[--qsize];
	min_heapify(0);
}

void enq(int s, int t)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].s = s, que[i].t = t;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define INF 0x10101010
#define MAX 1503
int n, dmax, sz;
short hi[MAX], *to[MAX], *dt[MAX];		// to, dist
short hi2[MAX], *to2[MAX];
int   dist[MAX];
char  mk[MAX], ans[MAX];
short q[2*MAX], top, tail;

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

void dijkstra(int start)
{
	int i, s, d, e;
	int max;

	memset(dist, INF, n << 2);
	memset(hi2, 0, n << 1);
	qsize = 0;
	dist[start] = 0, enq(start, 0);
	while (qsize) {
		s = que[0].s, deq();
		for (i = 0; i < hi[s]; i++) {
			e = to[s][i], d = dist[s] + dt[s][i];
			if (dist[e] < d) continue;
			if (dist[e] > d) hi2[e] = 0, dist[e] = d, enq(e, d);
			to2[e][hi2[e]++] = s;
		}
	}

	top = 0, tail = 0;
	max = 0; for (i = 1; i < n; i++) {
		if (dist[i] > max) max = dist[i], q[0] = i, tail = 1;
		else if (dist[i] == max) q[tail++] = i;
	}

	if (max < dmax) return;
	if (max > dmax) { dmax = max, sz = n-1, memset(ans, 0, n); }

	memset(mk, 0, n);
	while (top < tail) {
		s = q[top++];
		if (mk[s]) continue;
		mk[s] = 1;
		for (i = 0; i < hi2[s]; i++) {
			e = to2[s][i];
			if (mk[e]) continue;
			q[tail++] = e;
			if (!ans[s]) sz--, ans[s] = 1;
			if (!ans[e]) sz--, ans[e] = 1;
		}
	}
}

int main()
{
	int i, j, r, k, s, t, d;
	short *memo;
	
	n = in()+1, r = in();
	memo = malloc(r*6);
	j = 0, i = r; while (i--) {
		memo[j++] = s = in(), hi[s]++;
		memo[j++] = t = in(), hi[t]++;
		memo[j++] = in();
		
	}
	for (s = 0; s < n; s++) if (hi[s]) {
		to [s] = malloc(hi[s] << 1), dt[s] = malloc(hi[s] << 1);
		to2[s] = malloc(hi[s] << 1);
	}
	
	memset(hi, 0, n << 1);
	j = 0; while (r--) {
		s = memo[j++], t = memo[j++], d = memo[j++];
		k = hi[s]++, to[s][k] = t, dt[s][k] = d;
		k = hi[t]++, to[t][k] = s, dt[t][k] = d;
	}
	free(memo);

	sz = k = n-1; if (k > 100) k >>= 1;
	for (s = 1; s <= k && sz; s++) dijkstra(s);

	printf("%d\n", sz);
	if (sz) for (s = 1; s < n; s++) if (!ans[s]) printf("%d\n", s);
	return 0;
}
