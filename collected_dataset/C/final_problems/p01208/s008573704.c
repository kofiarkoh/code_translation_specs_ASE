// AOJ 2085 Turn Left
// 2018.3.24 bal4u

#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <math.h>

// 文字列のハッシュ関数
#define HASHSIZ 3001
typedef struct { char *s; int id; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;

int insert(char *s, int id)
{
	unsigned long long i;
	char *p;
	HASH *tp;

	i = 0, p = s; while (*p) i = (i<<5) + *p++;
	tp = hash + (int)(i % HASHSIZ);
	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->id;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, tp->id = id;
	return -1;
}

#define QMAX 1000
typedef struct { short s, p; double t; int k; } QUE;
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
		que[l].t == que[i].t && que[l].k < que[i].k)) min = l; else min = i;
	if (r < qsize && (que[r].t < que[min].t ||
		que[r].t == que[min].t && que[r].k < que[min].k)) min = r;
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

void enq(int s, int p, double t, int k)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].s = s, que[i].p = p, que[i].t = t, que[i].k = k;
	while (i > 0 && (que[min = PARENT(i)].t > que[i].t ||
		que[min].t == que[i].t && que[min].k > que[i].k)) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

typedef struct { int x, y; } PP;

#define EPS         1e-6
#define EQ(a,b)		(fabs((a)-(b))<EPS)
double dist_inter(PP p1, PP p2) { return hypot(p1.x-p2.x, p1.y-p2.y); };
PP vsub(PP p1, PP p2) { PP r; r.x = p1.x - p2.x, r.y = p1.y - p2.y; return r; }
long long cross(PP a, PP b) { return (long long)a.x * b.y - (long long)a.y * b.x; }
long long dot(PP a, PP b) { return (long long)a.x * b.x + (long long)a.y * b.y; }
long long norm(PP a) { return (long long)a.x * a.x + (long long)a.y * a.y; }
int ccw(PP p0, PP p1, PP p2) { PP a, b;	long long t;
	a = vsub(p1, p0), b = vsub(p2, p0), t = cross(a, b);
	if (t >  0) return 1;	if (t < 0) return -1;
	if (dot(a, b) < 0) return 2; if (norm(a) < norm(b)) return -2;
	return 0;
}

#define INF 0x45454545
typedef struct { PP p; short hi, to[10]; } T;
T      inter[1002]; int m;
char   name[1002][40];
double dist[1002];
char   buf[100], *p;
char   mk[1002][1002];

int in()
{
	int n = 0;
	while (*p >= '0') n = 10*n + (*p++ & 0xf);
	p++;
	return n;
}

int dijkstra(int start, int goal)
{
	int    i, s, p, e, k, a, ans;
	double t, nt;

	memset(mk, 0, sizeof(mk));
	qsize = 0;
	for (i = 0; i < inter[start].hi; i++) {
		e = inter[start].to[i];
		enq(e, start, dist_inter(inter[start].p, inter[e].p), 1);
	}

	ans = -1;
	while (qsize) {
		s = que[0].s, p = que[0].p, t = que[0].t, k = que[0].k, deq();

		if (s == goal) { ans = k+1; break; }

		if (mk[p][s]) continue;
		mk[p][s] = 1;

		for (i = 0; i < inter[s].hi; i++) {
			e = inter[s].to[i];
			a = ccw(inter[p].p, inter[s].p, inter[e].p);
			if (a == 1 || a == -2) {
				nt = t + dist_inter(inter[s].p, inter[e].p);
				if (mk[s][e]) continue;
				dist[e] = nt; enq(e, s, nt, k+1);
			}
		}
	}
	return ans;
}

int main()
{
	int m, n, i, a, b, k, start, goal;
	char *q;

	while (fgets(p=buf, 20, stdin) && *p != '0') {
		m = in(), n = in();
		memset(hash, 0, sizeof(hash));

		for (i = 0; i < m; i++) {
			fgets(p=name[i], 40, stdin);
			q = p; while (*p > ' ') p++; *p++ = 0;
			insert(q, i);
			inter[i].p.x = in(), inter[i].p.y = in(), inter[i].hi = 0;
		}

		for (i = 0;i < n; i++) {
			fgets(p=buf, 100, stdin);
			q = p; while (*p > ' ') p++; *p++ = 0;
			a = insert(q, 0);
			q = p; while (*p > ' ') p++; *p++ = 0;
			b = insert(q, 0);
			k = inter[a].hi++; inter[a].to[k] = b;
			k = inter[b].hi++; inter[b].to[k] = a;
		}
		fgets(p=buf, 100, stdin);
		q = p; while (*p > ' ') p++; *p++ = 0;
		start = insert(q, 0);
		q = p; while (*p > ' ') p++; *p++ = 0;
		goal = insert(q, 0);

		k = dijkstra(start, goal);
		if (k >= 0) printf("%d\n", k);
		else puts("impossible");
	}
	return 0;
}
