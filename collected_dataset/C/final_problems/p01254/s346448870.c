// AOJ 2135 Reverse a Road
// 2018.2.26 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define QMAX 15000
typedef struct { short t, s, r; } QUE;
QUE que[QMAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;
	QUE qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && (que[l].t < que[i].t ||
		que[l].t == que[i].t && que[l].r < que[i].r)) min = l; else min = i;
	if (r < qsize && (que[r].t < que[min].t ||
		que[r].t == que[min].t && que[r].r < que[min].r)) min = r;
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

void enq(int s, int t, int r)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].s = s, que[i].t = t, que[i].r = r;
	while (i > 0 && (que[min = PARENT(i)].t > que[i].t ||
		   que[min].t == que[i].t && que[min].r > que[i].r)) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

short hi[1002], to[1002][100], no[1002][100];
char mk[1002][1002][2];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = 10*n + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int dijkstra(int *rno, int start, int goal)
{
	int i, s, t, r, e, k;

	memset(mk, 0, sizeof(mk));
	qsize = 0;
	enq(start, 0, 0);
	while (qsize) {
		s = que[0].s, t = que[0].t, r = que[0].r, deq();
		if (s == goal) { *rno = r; return t; }

		if (r == 0) {
			for (i = 0; i < hi[s]; i++) {
				e = to[s][i], k = no[s][i];
				if (k) {
					if (!mk[e][s][1]) mk[e][s][1] = 1, enq(e, t+1, k);
				} else {
					if (!mk[s][e][0]) mk[s][e][0] = 1, enq(e, t+1, 0);
				}
			}
		} else {
			for (i = 0; i < hi[s]; i++) {
				e = to[s][i], k = no[s][i];
				if (k || mk[s][e][1]) continue;
				mk[s][e][1] = 1, enq(e, t+1, r);
			}
		}
	}
	return -1;
}

int main()
{
	int N, S, T, M, a, b, i, k, ans;

	while (N = in()) {
		N++, S = in(), T = in();
		M = in();

		memset(hi, 0, N << 1);
		memset(mk, 0, sizeof(mk));
		for (i = 1; i <= M; i++) {
			a = in(), b = in();
			if (mk[a][b][0]) continue;
			mk[a][b][0] = 1;
			k = hi[a]++, to[a][k] = b, no[a][k] = 0;
			k = hi[b]++, to[b][k] = a, no[b][k] = i;
		}

		ans = dijkstra(&k, S, T);
		printf("%d %d\n", ans, k);
	}
	return 0;
}
