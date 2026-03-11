// AOJ 2193: The Door into Summer
// 2018.1.7 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX 100003
/* UNION-FIND library */
int p[MAX], rank[MAX];

//void make_set(int x) { p[x] = x, rank[x] = 0; }
void link(int x, int y) {
	if (rank[x] > rank[y]) p[y] = x;
	else { p[x] = y; if (rank[x] == rank[y]) rank[y] = rank[y] + 1;	}
}
int find_set(int x) { if (x != p[x]) p[x] = find_set(p[x]);	return p[x]; }
void union_set(int x, int y) { link(find_set(x), find_set(y)); }

#define QMAX 15000
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
//	if (qsize == 0) return 0;
//	*n = que[0].node, *t = que[0].t, *c = que[0].c;
	que[0] = que[--qsize];
	min_heapify(0);
//	return 1;
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
int n;
int to[MAX][20]; char hi[MAX];
int goal[MAX], gsz;
int lpos[MAX], lsz;
int gdist[MAX], ndist[MAX], ldist[MAX];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

void dijkstra(int *dist, int *start, int sz)
{
	int i, s, d, e;

	memset(dist, INF, n << 2);
	qsize = 0;
	for (i = 0; i < sz; i++) dist[start[i]] = 0, enq(start[i], 0);
	while (qsize) {
		s = que[0].s, d = que[0].t, deq();
		for (i = 0; i < hi[s]; i++) {
			e = to[s][i];
			if (d+1 < dist[e]) dist[e] = d+1, enq(e, d+1);
		}
	}
}

int main()
{
	int cno, m, npos, i, a, b, c, ans;

	cno = in();
	while (cno--) {
		n = in()+1, m = in();

		memset(hi, 0, n);
		memset(rank, 0, n<<2);			// union set, <<2 = 4 = sizeof(int)
		for (i = 0; i < n; i++) p[i] = i;
		gsz = lsz = 0;

		npos = in(), a = in();	// initial position
		goal[gsz++] = 0;
		lpos[lsz++] = a;

		for (i = 0; i < m; i++) {
			a = in(), b = in(), c = getchar_unlocked(), getchar_unlocked();
			if (c == 'N') to[a][hi[a]++] = b, to[b][hi[b]++] = a;
			else union_set(a, b);
		}

		a = find_set(0), b = find_set(lpos[0]);
		if (find_set(0) == b) { puts("0"); continue; }
		for (i = 0; i < n; i++) {
			if ((c = find_set(i)) == a) goal[gsz++] = i;
			if ( c                == b) lpos[lsz++] = i;
		}

		dijkstra(gdist, goal, gsz);
		dijkstra(ldist, lpos, lsz);
		lpos[0] = npos;
		dijkstra(ndist, lpos, 1);

		ans = INF;
		for (i = 1; i < n; i++) {
			a = gdist[i]+ndist[i]+ldist[i];
			if (a < ans) ans = a;
		}
		printf("%d\n", ans);
	}
	return 0;
}

