// AOJ 2050 Dig or Climb
// 2018.3.2 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define QMAX 1000
typedef struct { double t; short s; } QUE;
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

void enq(int s, double t)
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

#define EPS		1e-8
#define EQ(a,b)	(fabs((a)-(b))<EPS)

int x[1001], y[1001];
int sz;
short lim[2000], hi[2000], *to[2000]; double *tm[2000];
char mk[2000];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	if (c == '-') {
		c = getchar_unlocked();
		do n = 10*n + (c & 0xf), c = getchar_unlocked();
		while (c >= '0');
		return -n;
	}
	do n = 10*n + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

double dist(double x1, double y1, double x2, double y2) { return hypot(x1-x2, y1-y2); }

double dijkstra(int n, int start, int goal)
{
	int i, s, e;
	double t = 0;

	memset(mk, 0, n);
	qsize = 0;
	enq(start, 0);
	while (qsize) {
		s = que[0].s, t = que[0].t, deq();
		if (s == goal) break;
		if (mk[s]) continue;
		mk[s] = 1;
		for (i = 0; i < hi[s]; i++) {
			e = to[s][i];
			if (mk[e]) continue;
			enq(e, t+tm[s][i]);
		}
	}
	return t;
}

void pushback(int a, int b, double t)
{
	int i;

	if (lim[a] == 0) {
		lim[a] = 2, hi[a] = 0;
		to[a] = malloc(sizeof(short) *lim[a]);
		tm[a] = malloc(sizeof(double)*lim[a]);
	} else if (hi[a] == lim[a]) {
		lim[a] <<= 1;
		to[a] = realloc(to[a], sizeof(short) *lim[a]);
		tm[a] = realloc(tm[a], sizeof(double)*lim[a]);
	}
	i = hi[a]++;
	to[a][i] = b, tm[a][i] = t;
}
		
int main()
{
	int n, vw, vc, i, j, k;
	double d, xx, yy;

	while (n = in()) {
		vw = in(), vc = in();
		memset(lim, 0, (3*n) << 1);
		memset(hi,  0, (3*n) << 1);
		for (i = 0; i < n; i++) {
			x[i] = in(), y[i] = in();
			if (i == 0) continue;
			d = dist(x[i-1], y[i-1], x[i], y[i]);
			pushback(i-1, i, d/vw);
		}
		sz = n;
		for (i = 0; i < n-1; i++) {
			if (y[i+1] <= y[i]) continue;
			for (j = i+1; j < n; j++) {
				if (y[j] == y[i]) {
					d = dist(x[i], y[i], x[j], y[j]);
					pushback(i, j, d/vc);
					break;
				} else if (j < n-1 && y[j] > y[i] && y[j+1] < y[i]) {
					k = sz++;
					yy = y[i];
					xx = (yy-y[j])*(x[j+1]-x[j])/((double)(y[j+1]-y[j])) + x[j];
					d = dist(x[i], y[i], xx, yy);
					pushback(i, k, d/vc);
					d = dist(x[j+1], y[j+1], xx, yy);
					pushback(k, j+1, d/vw);
					break;
				}
			}
		}
		for (i = 1; i < n; i++) {
			if (y[i-1] <= y[i]) continue;
			for (j = i-1; j >= 0; j--) {
				if (y[j] == y[i]) {
					d = dist(x[i], y[i], x[j], y[j]);
					pushback(j, i, d/vc);
					break;
				} else if (j > 0 && y[j] > y[i] && y[j-1] < y[i]) {
					k = sz++;
					yy = y[i];
					xx = (yy-y[j])*(x[j-1]-x[j])/((double)(y[j-1]-y[j])) + x[j];
					d = dist(x[i], y[i], xx, yy);
					pushback(k, i, d/vc);
					d = dist(x[j-1], y[j-1], xx, yy);
					pushback(j-1, k, d/vw);
					break;
				}
			}
		}
		printf("%.6lf\n", dijkstra(sz, 0, n-1));
		for (i = 0; i < sz; i++) {
			if (hi[i]) free(to[i]), free(tm[i]);
		}
	}
	return 0;
}
