// Aizu 0362: Trampoline
// 2017.9.27 bal4u@uu

#include <stdio.h>

#define MAX 300005
typedef struct { int t, b; } QUE;
QUE que[MAX]; int qsize;

#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

void min_heapify(int i)
{
  int l, r;
  int min;

  l = LEFT(i), r = RIGHT(i);
  if (l < qsize && que[l].t < que[i].t) min = l; else min = i;
  if (r < qsize && que[r].t < que[min].t) min = r;
  if (min != i) {
    QUE t = que[i]; que[i] = que[min]; que[min] = t;
    min_heapify(min);
  }
}

int deq1(int *t, int *b)
{
  if (qsize == 0) return 0;
  *t = que[0].t, *b = que[0].b;
  que[0] = que[--qsize];
  min_heapify(0);
  return 1;
}

void enq1(int t, int b)
{
  int i, ii;

  i = qsize++;
  que[i].t = t, que[i].b = b;
  while (i > 0 && que[ii = PARENT(i)].t > que[i].t) {
    QUE tt = que[i]; que[i] = que[ii]; que[ii] = tt;
    i = ii;
  }
}

void max_heapify(int i)
{
	int l, r, largest;
	l = LEFT(i), r = RIGHT(i);
	if (l < qsize && que[l].t > que[i].t) largest = l;
	else largest = i;
	if (r < qsize && que[r].t > que[largest].t)	largest = r;
	if (largest != i) {
		QUE t = que[i]; que[i] = que[largest], que[largest] = t;
		max_heapify(largest);
	}
}

int deq2(int *t, int *b)
{
	if (qsize == 0) return 0;
	*t = que[0].t, *b = que[0].b;
	que[0] = que[--qsize];
	max_heapify(0);
	return 1;
}

void enq2(int t, int b)
{
	int i, max;
	i = qsize++;
	que[i].t = t, que[i].b = b;
	while (i > 0 && que[max = PARENT(i)].t < que[i].t) {
		QUE t = que[i]; que[i] = que[max], que[max] = t;
		i = max;
	}
}

int d[MAX];

int main()
{
	int n, i, j, k, b;

	scanf("%d", &n);
	qsize = 0;					// ????????????????????£?????\???????????????
	for (j = 0, i = 0; i < n; i++, j += 10)	{
		scanf("%d", &k); d[i] = k;
		enq1(j, j+k);
	}
	for (j = 0, i = 0; i < n; i++, j += 10) {
		k = que[0].t, b = que[0].b;
		if (j < k) goto NO;
		if (i < n-1) {
			if (j+10 > b) while (deq1(&k, &b) && j+10 > b);
			if (!(k <= j && j+10 <= b)) goto NO;
		}
	}

	qsize = 0;
	for (j = (n-1)*10, i = n-1; i >= 0; i--, j -= 10) enq2(j, j-d[i]);
	for (j = (n-1)*10, i = n-1; i >= 0; i--, j -= 10) {
		k = que[0].t, b = que[0].b;
		if (j > k) goto NO;
		if (i > 1) {
			if (j-10 < b) while (deq2(&k, &b) && j-10 < b);
			if (!(b <= j-10 && j <= k)) goto NO;
		}
	}
	puts("yes");
	return 0;
NO: puts("no");
	return 0;
}