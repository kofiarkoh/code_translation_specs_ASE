// AOJ 0387 Taxi
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
#define QSIZE 300010
int que[QSIZE + 5];
int qsize;

void max_heapify(int i)
{
	int l, r, max;
	int qt;

	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l] > que[i]) max = l; else max = i;
	if (r < qsize && que[r] > que[max]) max = r;
	if (max != i) {
		qt = que[i], que[i] = que[max], que[max] = qt;
		max_heapify(max);
	}
}

void deq()
{
	que[0] = que[--qsize];
	max_heapify(0);
}

void enq(int c)
{
	int i, max;
	int qt;

	i = qsize++;
	que[i] = c;
	while (i > 0 && que[max = PARENT(i)] < que[i]) {
		qt = que[i]; que[i] = que[max], que[max] = qt;
		i = max;
	}
}


// 本問題関連
int c[300010];
int top[300010], sz[300010]; int N;

int main()
{
	int i, k, M;
	unsigned ans;

	N = in(), k = 0;
	for (i = 0; i < N; i++) {
		sz[i] = M = in();
		top[i] = k;
		while (M--) c[k++] = in();
	}
	
	// 各列のデータを逆順に優先度付きキューに追加
	ans = 0, qsize = 0;
	for (i = N - 1; i >= 0; i--) {
		M = sz[i], k = top[i];
		while (M--) enq(c[k++]);
		ans += (unsigned)que[0];
		deq();
	}
	printf("%u\n", ans);
	return (0);
}

