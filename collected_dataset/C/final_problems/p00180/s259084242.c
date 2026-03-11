// Aizu Vol-1 0180: Demolition of Bridges
// 2017.8.9

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INF 0x77777777
#define MAX 100
#define EMAX 500
int n;
typedef struct { int b, c; } ND;
ND d[MAX + 2][MAX + 2];
int w[MAX + 2];
int visited[MAX];

int qsize;
typedef struct { int a, b, c; } QUE;
QUE que[EMAX + 1];

#define PARENT(i) ((i)/2)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

static void min_heapify(int i)
{
	int l, r, smallest;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].c < que[i].c) smallest = l; else smallest = i;
	if (r < qsize && que[r].c < que[smallest].c) smallest = r;
	if (smallest != i) {
		QUE t;
		memcpy(&t, que + i, sizeof(QUE)), memcpy(que + i, que + smallest, sizeof(QUE));
		memcpy(que + smallest, &t, sizeof(QUE));
		min_heapify(smallest);
	}
}

int deq(QUE *q)
{
	if (qsize == 0) return -1;
	memcpy(q, &que[0], sizeof(QUE));
	memcpy(que, que + --qsize, sizeof(QUE));
	min_heapify(0);
	return 1;
}

void enq(QUE *q)
{
	int i;

	i = qsize++;
	memcpy(que + i, q, sizeof(QUE));
	while (i > 0 && que[PARENT(i)].c > que[i].c) {
		QUE t;
		memcpy(&t, que + i, sizeof(QUE)), memcpy(que + i, que + PARENT(i), sizeof(QUE));
		memcpy(que + PARENT(i), &t, sizeof(QUE));
		i = PARENT(i);
	}
}


int main()
{
	int m, a, b, c;
	int i, k, ans;
	QUE q;

	while (1) {
		scanf("%d%d", &n, &m); if (!n) return 0;
		memset(w, 0, sizeof(w));
		for (i = 0; i < m; i++) {
			scanf("%d%d%d", &a, &b, &c);
			d[a][w[a]].b = b, d[a][w[a]++].c = c;
			d[b][w[b]].b = a, d[b][w[b]++].c = c;
		}
		memset(visited, 0, sizeof(visited));
		ans = 0, visited[0] = 1, k = 1, qsize = 0;
		for (i = 0; i < w[0]; i++) q.a = 0, q.b = d[0][i].b, q.c = d[0][i].c, enq(&q);
		while (k < n) {
			do deq(&q);
			while (visited[q.a] && visited[q.b]);
			k++;
			a = visited[q.a] ? q.b : q.a;
			visited[a] = 1, ans += q.c;
			for (i = 0; i < w[a]; i++) {
				b = d[a][i].b;
				if (!visited[b]) q.a = a, q.b = b, q.c = d[a][i].c, enq(&q);
			}

		}
		printf("%d\n", ans);
	}
	return 0;
}