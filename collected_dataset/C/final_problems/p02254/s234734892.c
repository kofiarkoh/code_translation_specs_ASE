// AOJ ALDS1_15_D Huffman Coding
// 2019.3.9 bal4u

#include <stdio.h>
#include <stdlib.h>

// 優先度付きキュー
#define MAX 60    // 2*26
typedef struct { int id, f; } QUE;
QUE que[MAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
	int l, r, min;
	QUE qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].f < que[i].f) min = l; else min = i;
	if (r < qsize && que[r].f < que[min].f) min = r;
	if (min != i) {
		qt = que[i]; que[i] = que[min]; que[min] = qt;
		min_heapify(min);
	}
}

void deq()
{
	que[0] = que[--qsize];
	min_heapify(0);
}

void enq(int id, int f)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].id = id, que[i].f = f;
	while (i > 0 && que[min = PARENT(i)].f > que[i].f) {
		qt = que[i]; que[i] = que[min]; que[min] = qt;
		i = min;
	}
}

typedef struct { int f; int left, right; } T;
T tbl[MAX]; int sz;
char buf[100003];

int count(int id, int level)
{
	if (id < 26) return tbl[id].f*level;
	return count(tbl[id].left, level + 1) + count(tbl[id].right, level + 1);
}

int main()
{
	int i, l_id, l_f, r_id, r_f;
	char *p;

	fgets(p=buf, 100003, stdin);
	while (*p >= 'a') tbl[*p-'a'].f++, p++;   // 文字ごとの出現頻度
	for (i = 0; i < 26; i++) if (tbl[i].f) enq(i, tbl[i].f);  // Qに入れる
	if (qsize == 1) { printf("%d\n", que[0].f); return 0; }
	sz = 26;
	while (1) {
		l_id = que[0].id, l_f = que[0].f, deq();
		if (!qsize) break;
		r_id = que[0].id, r_f = que[0].f, deq();
		tbl[sz].f = l_f + r_f, tbl[sz].left = l_id, tbl[sz].right = r_id;
		enq(sz, tbl[sz].f);
		sz++;
	}
	printf("%d\n", count(l_id, 0));
	return 0;
}
