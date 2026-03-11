// AOJ 2425: A Holiday of Miss Brute Force
// 2017.12.30 bal4u@uu

#include <stdio.h>
#include <string.h>
 
#define QMAX 10000
typedef struct { int t, x, y, s; } QUE;
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

void enq(int x, int y, int s, int t)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].x = x, que[i].y = y, que[i].s = s, que[i].t = t;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define INF 0x30303030
#define BASE 100

char map[202][202];
int dist[202][202][6];
int mv[2][6][2] = {{{1,0},{0,1},{-1,1},{-1,0},{-1,-1},{0,-1}},
				   {{1,0},{1,1},{ 0,1},{-1,0},{ 0,-1},{1,-1}}};
char buf[30], *p;

int in()
{
	int n = 0;
	if (*p == '-') { p++; while (*p >= '0') n = 10*n + (*p++ & 0xf); n = -n; }
	else while (*p >= '0') n = 10*n + (*p++ & 0xf);
	p++;
	return n;
}

void dijkstra(int sx, int sy, int lx, int ly)
{
	int i, x, y, t, x2, y2, t2, d, e;

	memset(dist, INF, sizeof(dist));
	dist[sy][sx][0] = 0;
	qsize = 0;
	enq(sx, sy, 0, 0);
	while (qsize) {
		x = que[0].x, y = que[0].y, t = que[0].s, deq();

		d = (x-BASE)*(y-BASE)*t; if (d < 0) d = -d;	d %= 6;
		t2 = t+1; if (t2 == 6) t2 = 0;

		// ????????´????????????
		if (dist[y][x][t2] > dist[y][x][t]+1) {
			dist[y][x][t2] = dist[y][x][t]+1;
			enq(x, y, t2, dist[y][x][t2]);
		}

		// ?????????????§??????????
		for (i = 0; i < 6; i++) {
			x2 = x + mv[x&1][i][1], y2 = y + mv[x&1][i][0];
			e = dist[y][x][t] + (i!=d);
			if (-lx <= x2-BASE && x2-BASE <= lx && -ly <= y2-BASE && y2-BASE <= ly && !map[y2][x2]) {
				if (dist[y2][x2][t2] > e) {
					dist[y2][x2][t2] = e;
					enq(x2, y2, t2, e);
				}
			}
		}
	}
}

int main()
{
	int sx, sy, gx, gy, n, lx, ly, i, x, y, ans;

	fgets(p=buf, 30, stdin);
	sx = BASE+in(), sy = BASE+in(), gx = BASE+in(), gy = BASE+in();
	fgets(p=buf, 10, stdin), n = in();
	while (n--) {
		fgets(p=buf, 15, stdin), x = BASE+in(), y = BASE+in();
		map[y][x] = 1;
	}
	fgets(p=buf, 15, stdin), lx = in(), ly = in();

	dijkstra(sx, sy, lx, ly);
	ans = INF;
	for (i = 0; i < 6; i++) if (ans > dist[gy][gx][i]) ans = dist[gy][gx][i];
	if (ans == INF) puts("-1");
	else printf("%d\n", ans);
	return 0;
}