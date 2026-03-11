// AOJ 0396: Treasure Map
// 2018.3.20 bal4u

#include <stdio.h>
#include <stdlib.h>

// 高速数値入出力
#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()   // 非負整数の入力
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

// 優先度付きキュー（最大値）
#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)
#define QSIZE 1005
int que[2][QSIZE + 5];
int qsize[2];

void max_heapify(int id, int i)
{
	int l, r, max;
	int qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize[id] && que[id][l] > que[id][i]) max = l; else max = i;
	if (r < qsize[id] && que[id][r] > que[id][max]) max = r;
	if (max != i) {
		qt = que[id][i], que[id][i] = que[id][max], que[id][max] = qt;
		max_heapify(id, max);
	}
}

void deq(int id)
{
	que[id][0] = que[id][--qsize[id]];
	max_heapify(id, 0);
}

void enq(int id, int c)
{
	int i, max;
	int qt;

	i = qsize[id]++;
	que[id][i] = c;
	while (i > 0 && que[id][max = PARENT(i)] < que[id][i]) {
		qt = que[id][i]; que[id][i] = que[id][max], que[id][max] = qt;
		i = max;
	}
}


// 本問題関連
int W, H;
int a[1005], b[1005];

int cmp(const void *a, const void *b) { return *(int *)b - *(int *)a; }
int main()
{
	int i, j, t, id, sa, sb;

	W = in(), H = in(), sa = sb = 0;
	for (i = 0; i < W; i++) a[i] = in(), sa += a[i];
	for (j = 0; j < H; j++) b[j] = in(), sb += b[j];
	if (sa != sb) goto No;
	qsort(a, W, sizeof(int), cmp);
	qsort(b, H, sizeof(int), cmp);
	if (W == H && a[0] == a[W - 1] && b[0] == b[H - 1]) goto Yes;
	qsize[0] = qsize[1] = 0;
	for (j = 0; b[j]; j++) enq(0, b[j]);
	for (id = i = 0; t = a[i]; i++, id = !id) {
		while (t > 0 && qsize[id] > 0) {
			j = que[id][0], deq(id), t--;
			if (j > 1) enq(!id, j - 1);
		}
		if (t > 0) goto No;
		while (qsize[id]) j = que[id][0], deq(id), enq(!id, j);
	}
Yes:puts("1");
	return 0;
No: puts("0");
	return 0;
}

