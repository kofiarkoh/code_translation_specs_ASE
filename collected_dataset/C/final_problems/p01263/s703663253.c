/*
   AOJ 2144 Reaction
   2019.3.26 bal4u
   最小費用流で解く
   問題では最大売値を求めるので、コストをマイナスすることで最小化。
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//// 数値入力
#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int ins(char *s)  // 文字列の入力　スペース以下の文字で入力終了
{
	char *p = s;
	do *s = gc();
	while (*s++ > ' ');
	*--s = 0;
	return s - p;
}

//// 文字列のハッシュ関数
// 文字列からそのIDを知る。本問題では異なる文字列（アイテム）
//    が100未満なので、使わないくても全然OK
#define HASHSIZ 1999 
typedef struct { char *s; int id; } HASH;
HASH hash[HASHSIZ + 2], *hashend = hash + HASHSIZ;

int insert(char *s, int id)
{
	unsigned long long i;
	char *p;
	HASH *tp;

	i = 0, p = s; while (*p) i = (i << 5) + *p++;
	tp = hash + (int)(i % HASHSIZ);
	while (tp->s != NULL) {
		if (!strcmp(tp->s, s)) return tp->id;
		if (++tp == hashend) tp = hash;
	}
	tp->s = s, tp->id = id;
	return -1;
}


//// 最小費用流アルゴリズム
#define QMAX 30005
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
#define MAX 205
typedef struct { int to, rev; int cap, cost; } EDGE;
EDGE *edge[MAX]; int hi[MAX];
int V;				// 頂点数
int potential[MAX], dist[MAX], prevv[MAX], preve[MAX];

void add_edge(int from, int to, int cap, int cost)
{
	int f, t;
	EDGE *ep;

	f = hi[from]++, t = hi[to]++;
	ep = &edge[from][f], ep->to = to, ep->cap = cap, ep->cost = cost, ep->rev = t;
	ep = &edge[to][t], ep->to = from, ep->cap = 0, ep->cost = -cost, ep->rev = f;
}

int minCostFlow(int S, int T, int F)
{
	int i, v, nv, t, nt, ans;
	EDGE *ep;
	int first = 1;

	ans = 0;
//	memset(potential, 0, sizeof(int) * V);

	while (1) {
		memset(dist, INF, sizeof(int) * V);
		qsize = 0;
		enq(S, 0), dist[S] = 0;
		while (qsize) {
			v = que[0].s, t = que[0].t, deq();
			if (dist[v] < t) continue;
			for (i = 0; i < hi[v]; i++) {
				ep = &edge[v][i];
				if (ep->cap <= 0) continue;
				nv = ep->to;
				nt = t + ep->cost - potential[nv] + potential[v];
				if (dist[nv] > nt) {
					dist[nv] = nt;
					prevv[nv] = v, preve[nv] = i;
					enq(nv, nt);
				}
			}
		}

		if (!first) {
			for (v = 0; v < V; v++) potential[v] += dist[v];
		}
		first = 0;
		if (potential[T] >= 0) break;

		t = INF;
		for (v = T; v != S; v = prevv[v]) {
			ep = &edge[prevv[v]][preve[v]];
			if (t > ep->cap) t = ep->cap;
		}

		ans += t * potential[T];
		for (v = T; v != S; v = prevv[v]) {
			ep = &edge[prevv[v]][preve[v]];
			ep->cap -= t;
			edge[v][ep->rev].cap += t;
		}
	}
	return ans;
}

int M, P;    // マイナス球、プラス球の異なる種類数
int nm[105], np[105];
char item[105][15]; int nItem;
int price[105];     // 各アイテムの売値
int cost[105][105];

int main()
{
	int i, j, max;
	int source, sink, total;
	char buf[15];

	while (M = in()) {
		memset(hash, 0, sizeof(hash));

		// 入力処理
		P = in();
		total = 0; for (i = 0; i < M; i++) nm[i] = in(), total += nm[i];

		for (i = 0; i < P; i++) np[i] = in();

		j = in(), nItem = 0;
		while (j--) {
			ins(item[nItem]);
			insert(item[nItem], nItem);
			price[nItem++] = in();
		}

		memset(cost, 0, sizeof(cost));
		max = 0;
		j = in(); while (j--) {
			int fr = in() - 1, to = in() - 1, s = 0;
			int n = in();             // 生成するアイテム数
			while (n--) {
				ins(buf);
				i = insert(buf, 0);   // 登録したアイテムのIDが戻ってくるはず
				s += price[i];
			}
			cost[fr][to] = s;        // ひとつの反応で生まれる売値の合計
			if (s > max) max = s;
		}

		// 各枝に対する流量/コストの設定
		source = M + P, sink = source + 1, V = sink + 1;

		hi[source] = M, hi[sink] = P;
		for (i = 0; i < M; i++) hi[i] = P + 1;
		for (i = 0; i < P; i++) hi[i + M] = M + 1;
		for (i = 0; i < V; i++) edge[i] = malloc(sizeof(EDGE)*hi[i]);

		memset(hi, 0, sizeof(int) * V);
		memset(potential, 0, sizeof(int) * V);
		for (i = 0; i < M; i++) add_edge(source, i, nm[i], 0);    // sourceから
		for (i = 0; i < P; i++) add_edge(i + M, sink, np[i], 0);  // sinkへ
		for (i = 0; i < M; i++) {
			for (j = 0; j < P; j++) add_edge(i, j + M, total, -cost[i][j]);
			if (-cost[i][j] < potential[j + M]) potential[j + M] = -cost[i][j];
		}
		potential[sink] = -max;
		printf("%d\n", -minCostFlow(source, sink, total));
		for (i = 0; i < V; i++) free(edge[i]);
	}
	return 0;
}
