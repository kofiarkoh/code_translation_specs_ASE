// AOJ 0351: Quiet Town
// 2018.1.9 bal4u@uu

#include <stdio.h>
#include <string.h>

#define QMAX 10000
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

//int qmax;
void enq(int s, int t)
{
	int i, min;
	QUE qt;

	i = qsize++;	//if (qsize > qmax) qmax = qsize;
	que[i].s = s, que[i].t = t;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define W 1500
int n, sz;
short to[1501][W], dt[1501][W], hi[1501];	// to, dist, len
int dmax;									// max d
int tbl[1501][W]; short w[1501];
int dist[1501];							
short prev[1501];
char marked[1501];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int check(int s)
{
	int i, e, l, h;

//printf("check s %d\n", s+1);
	l = h = 0;
	for (i = 0; i < hi[s]; i++) {
		e = to[s][i];
//printf("s %d, i %d, w %d, dist %d --> e %d, dist %d\n",s+1,i,dt[s][i],dist[s],e+1,dist[e]); 
		if (!marked[e]) continue;
		if (dist[e] >= dist[s]) {
			if (dist[s] + dt[s][i] == dist[e]) h++;
		} else {
			if (dist[s] - dt[s][i] == dist[e]) l++;
		}
	}
//printf("!!!! low %d, hi %d\n", l, h);
	return l >= 1 && h >= 1; 
}

void dijkstra(int start)
{
	int i, s, d, e;
	int max;

	memset(dist, 0x33, n << 2);
	memset(prev, -1, n << 1);

	qsize = 0;
	dist[start] = 0, enq(start, 0);
	while (qsize) {
		s = que[0].s, deq();
		for (i = 0; i < hi[s]; i++) {
			e = to[s][i];
			d = dist[s] + dt[s][i];
			if (d < dist[e]) prev[e] = s, dist[e] = d, enq(e, d);
		}
	}
	max = 0; for (i = 0; i < n; i++) if (dist[i] > max) max = dist[i];
//printf("start %d, max %d, dmax %d\n", start+1, max, dmax);
	if (max > dmax) dmax = max;
	if (max == dmax) {
		memset(marked, 0, n);
		for (i = 0; i < n; i++) if (dist[i] == max) {
			for (s = i; s >= 0; s = prev[s]) if (!marked[s]) {
				marked[s] = 1, tbl[s][w[s]++] = max;
			}
		}
		for (s = 0; s < n; s++) if (!marked[s]) {
			if (check(s)) tbl[s][w[s]++] = max;
		}
	}
//for (s = 0; s < n; s++) printf("[%d] w %d, tbl %d,%d\n", s+1, w[s], tbl[s][0], tbl[s][1]);
}

//int hmax;

int main()
{
	int r, k, s, t, d;
	int *ans;

	n = in(), r = in();
	while (r--) {
		s = in()-1, t = in()-1, d = in();
		k = hi[s]++, to[s][k] = t, dt[s][k] = d;
		k = hi[t]++, to[t][k] = s, dt[t][k] = d;
	}
//for (s = 0; s < n; s++) if (hi[s] > hmax) hmax = hi[s];

	for (s = 0; s < n; s++) dijkstra(s);

	ans = dist;	for (s = 0; s < n; s++) {
		for (k = 0; ; k++) {
			if (k == w[s]) { ans[sz++] = s; break; }
			if (tbl[s][k] == dmax) break; 
		}
	}
	printf("%d\n", sz);
	for (k = 0; k < sz; k++) printf("%d\n", 1+ans[k]);

//	printf("hmax %d, qmax %d\n", hmax, qmax);
	return 0;
}
