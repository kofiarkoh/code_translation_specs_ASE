// AOJ 2419 Acrophobia
// 2018.4.8 bal4u

#include <stdio.h>
#include <string.h>

#define QMAX 10000
typedef struct { char r, c, ro; short t; } QUE;
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

void enq(int r, int c, int ro, int t)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].r = r, que[i].c = c, que[i].ro = ro, que[i].t = t;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define ABS(a)   ((a)>=0?(a):-(a))
#define INF  0x3030
int   W, H;
char  map[103][105];
char  roll[103][103]; int sz;
char  cost[103][103];
short dist[103][103][1<<5];
int   mv[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

int dijkstra(int sr, int sc, int gr, int gc, int all)
{
	int i, r, c, nr, nc, ro, t, nt, x;

	memset(dist, INF, sizeof(dist));
	qsize = 0;
	enq(sr, sc, 0, 0);
	while (qsize) {
		r = que[0].r, c = que[0].c, ro = que[0].ro, t = que[0].t, deq();
		if (r == gr && c == gc && ro == all) break;
		if (dist[r][c][ro] <= t) continue;
		dist[r][c][ro] = t;
		for (i = 0; i < 4; i++) {
			nr = r + mv[i][0], nc = c + mv[i][1];
			if (map[nr][nc] == '#') continue;
			nt = t + cost[r][c];
			if (dist[nr][nc][x=(ro|roll[nr][nc])] <= nt) continue;
			enq(nr, nc, x, nt);
		}
	}
	return t;
}

int main()
{
	int r, c, d, nr, nc;
	int sr, sc, gr, gc;
	char buf[20];

	fgets(buf, 20, stdin), sscanf(buf, "%d%d", &W, &H);
	memset(map, '#', sizeof(map));
	sz = 0; for (r = 1; r <= H; r++) {
		fgets(map[r]+1, 103, stdin);
		for (c = 1; c <= W; c++) {
			if      (map[r][c] == 'S') sr = r, sc = c;
			else if (map[r][c] == 'G') gr = r, gc = c;
			else if (map[r][c] == 'M') roll[r][c] |= 1<<sz++;
		}
		map[r][c] = '#';
	}
	memset(cost, 1, sizeof(cost));
	for (r = 1; r <= H; r++) for (c = 1; c <= W; c++) if (map[r][c] == '#') {
		for (nr = r-3; nr <= r+3; nr++) for (nc = c-3; nc <= c+3; nc++) {
			if (nr < 1 || nr > H || nc < 1 || nc > W) continue;
			d = ABS(nr-r); if (d < ABS(nc-c)) d = ABS(nc-c);
			d = 4-d;
			if (d > cost[nr][nc]) cost[nr][nc] = d;
		}
	}
	printf("%d\n", dijkstra(sr, sc, gr, gc, (1<<sz)-1));
	return 0;
}
