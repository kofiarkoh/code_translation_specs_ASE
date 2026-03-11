// Aizu 0212: Highway Express Bus
// 2017.9.9 bal4u@uu

#include <stdio.h>
#include <string.h>

#define INF 0x7fffffff
#define MAX 10000
typedef struct { int t, k, c; } QUE;
QUE que[MAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
  int l, r;
  int smallest;

  l = LEFT(i), r = RIGHT(i);
  if (l < qsize && que[l].t < que[i].t) smallest = l; else smallest = i;
  if (r < qsize && que[r].t < que[smallest].t) smallest = r;
  if (smallest != i) {
    QUE t = que[i]; que[i] = que[smallest]; que[smallest] = t;
    min_heapify(smallest);
  }
}

int deq(int *k, int *c)
{
  if (qsize == 0) return 0;
  *k = que[0].k, *c = que[0].c;
  que[0] = que[--qsize];
  min_heapify(0);
  return 1;
}

void enq(int k, int t, int c)
{
  int i, ii;

  i = qsize++;
  que[i].t = t, que[i].k = k, que[i].c = c;
  while (i > 0 && que[ii = PARENT(i)].t > que[i].t) {
    QUE tt = que[i]; que[i] = que[ii]; que[ii] = tt;
    i = ii;
  }
}

typedef struct { int len, to[102], f[102]; } TBL;
TBL tbl[102];
int node[102][11]; int size;
char visited[102][11];
int start, goal, cmax;

int search()
{
	int s, i, c, s2, f2, c2, k;
	TBL *p;

	qsize = 0;
	memset(node, -1, sizeof(node)), memset(visited, 0, sizeof(visited));
	node[start][cmax] = 0;
	enq(start, 0, cmax);
	while(deq(&s, &c)) {
		if (s == goal) continue;
		if (visited[s][c]) continue;
		visited[s][c] = 1;
		for (p = tbl+s, i = 0; i < p->len; i++) {
			s2 = p->to[i], f2 = p->f[i];
			if (visited[s2][c]) continue;
			k = node[s][c] + f2;
			if (node[s2][c] < 0 || k < node[s2][c]) node[s2][c] = k, enq(s2, k, c);
			if (c > 0) {
				c2 = c-1;
				if (visited[s2][c2]) continue;
				k = node[s][c] + f2 / 2;
				if (node[s2][c2] < 0 || k < node[s2][c2]) node[s2][c2] = k, enq(s2, k, c2);
			}
		}
	}
	return node[goal][0];
}

int main()
{
	int m, i, a, b, f;
	TBL *p;

	while (scanf("%d%d%d%d%d", &cmax, &size, &m, &start, &goal) && cmax) {
		memset(tbl, 0, sizeof(TBL)*(size+1));
		for (i = 0; i < m; i++) {
			scanf("%d%d%d", &a, &b, &f);
			p = tbl + a, p->to[p->len] = b, p->f[p->len] = f, p->len++;
			p = tbl + b, p->to[p->len] = a, p->f[p->len] = f, p->len++;
		}
		printf("%d\n", search());
	}
	return 0;
}