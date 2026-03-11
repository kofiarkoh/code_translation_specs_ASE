// AOJ 1048 Provident Housewife
// 2018.2.23 bal4u
 
#include <stdio.h>
#include <string.h>

#define HASHSIZ		2003
typedef struct { char *s; short id; } HASH;
HASH hash[HASHSIZ+2], *hashend = hash + HASHSIZ;			// Hash table

int insert(char *s, short id)
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

#define QMAX 10000
typedef struct { char s, e; int t, b, p; } QUE;
QUE que[QMAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;
	QUE qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && (que[l].p < que[i].p ||
		que[l].p == que[i].p && que[l].t < que[i].t)) min = l; else min = i;
	if (r < qsize && (que[r].p < que[min].p ||
		que[r].p == que[min].p && que[r].t < que[min].t)) min = r;
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

void enq(char s, char e, int t, int p, int bit)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].s = s, que[i].e = e, que[i].t = t, que[i].p = p, que[i].b = bit;
	while (i > 0 && (que[min = PARENT(i)].p > que[i].p ||
		que[min].p == que[i].p && que[min].t > que[i].t)) {
		qt = que[i], que[i] = que[min], que[min] = qt;
		i = min;
	}
}

#define INF 0x1010
#define MAX 10001

short price[1002], many[1002], shop[1002][11];
char name[1002][22]; int sz;
short list[16];
char buf[2800], *p;

short hi[12], to[12][12], tbl[12][12];
char buy[12][16], w[12];
char mk[12][12][1<<15];

int in()
{
	int n = 0;
	while (*p >= '0') n = 10*n + (*p++ & 0xf);
	p++;
	return n;
}

int dijkstra(int Q)
{
	int i, j, s, prev, e, p, dist, bit, np, nbit;
	int goal = (1 << Q)-1;

	memset(mk, 0, sizeof(mk));
	qsize = 0;
	enq(0, 0, 0, 0, 0);
	while (qsize) {
		s = que[0].e, prev = que[0].s;
		p = que[0].p, dist = que[0].t, bit = que[0].b, deq();
		if (s == 0 && bit == goal) return dist;

		if (mk[prev][s][bit]) continue;
		mk[prev][s][bit] = 1;

		for (i = 0; i < hi[s]; i++) {
			e = to[s][i];
			nbit = bit, np = p;
			for (j = 0; j < w[e]; j++) {
				int b = 1 << buy[e][j];
				if (nbit & b) continue;
				nbit |= b;
				np += price[list[buy[e][j]]];
			}
			if (mk[s][e][nbit]) continue;
			enq(s, e, dist+tbl[s][e], np, nbit);
		}
	}
	return -1;
}

int main()
{
	int N, Q, M, i, k, id, s, t, x;
	int pay;
	char *q;

	while (fgets(p=buf, 10, stdin) && *p != '0') {
		memset(hash, 0, sizeof(hash));
		memset(many, 0, sizeof(many));

		N = in();
		sz = 0; for (i = 1; i <= N; i++) {
			fgets(p=buf, sizeof(buf), stdin);
			k = in(); while (k--) {
				q = p; while (*p > ' ') p++; *p++ = 0, strcpy(name[sz], q);
				id = insert(name[sz], sz);
				if (id < 0) id = sz++;
				x = in();
				if (many[id] == 0 || x < price[id])
					many[id] = 1, shop[id][0] = i, price[id] = x;
				else if (x == price[id]) shop[id][many[id]++] = i;
			}
		}

		memset(w, 0, sizeof(w));
		fgets(p=buf, 10, stdin), Q = in();
		pay = 0;
		for (i = 0; i < Q; i++) {
			fgets(p=buf, 30, stdin); while (*p > ' ') p++; *p = 0;
			if (pay < 0) continue;
			id = insert(buf, -1);
			if (id < 0) pay = -1;
			else {
				pay += price[id];
				list[i] = id;
				for (k = 0; k < many[id]; k++) {
					x = shop[id][k];
					buy[x][w[x]++] = i;
				}
			}
		}

		memset(tbl, INF, sizeof(tbl));
		fgets(p=buf, 10, stdin), M = in();
		while (M--) {
			fgets(p=buf, 30, stdin), s = in(), t = in(), x = in();
			if (pay >= 0 && x < tbl[s][t]) tbl[s][t] = tbl[t][s] = x;
		}

		if (pay >= 0) {
			memset(hi, 0, sizeof(hi));
			for (s = 0; s <= N; s++) for (t = s+1; t <= N; t++) {
				if (tbl[s][t] < INF) {
					to[s][hi[s]++] = t;
					to[t][hi[t]++] = s;
				}
			}

			x = dijkstra(Q);
			if (x < 0) pay = -1;
			else printf("%d %d\n", pay, x);
		}
		
		if (pay < 0) puts("impossible");
	}
	return 0;
}

