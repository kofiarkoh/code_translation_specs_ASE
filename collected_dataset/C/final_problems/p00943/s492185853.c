// AOJ 1364: Routing a Marathon Race
// 2018.1.22 bal4u@uu

#include <stdio.h>

#define QMAX 5000
typedef struct { int u, t, p; long long b; } QUE;
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

void enq(int u, int t, int p, long long b)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].t = t, que[i].u = u, que[i].p = p, que[i].b = b;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

int to[41][41], hi[41]; 
int c[41];
char used[41][41];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, m, i, j, k, u, v, p, w, s;
	long long b, b2;

	n = in(), m = in();
	for (i = 1; i <= n; i++) c[i] = in();
	while (m--) {
		i = in(), j = in();
		k = hi[i]++, to[i][k] = j;
		k = hi[j]++, to[j][k] = i;
	}

	qsize = 0;
	b = 1 << 1, m = c[1];
	for (i = 0; i < hi[1]; i++) v = to[1][i], m += c[v], b |= 1LL << v;
	enq(1, m, 0, b);
	while (qsize) {
		u = que[0].u, m = que[0].t, p = que[0].p, b = que[0].b, deq();
		if (u == n) break;

		if (used[p][u]) continue;
		used[p][u] = 1;
		for (i = 0; i < hi[u]; i++) {
			v = to[u][i];
			if (used[u][v]) continue;
			for (s = 0, b2 = b, k = 0; k < hi[v]; k++) {
				w = to[v][k];
				if (!((b2 >> w) & 1)) s += c[w], b2 |= 1LL << w;
			}
			enq(v, m+s, u, b2);
		}
	}
	printf("%d\n", m);
	return 0;
}

