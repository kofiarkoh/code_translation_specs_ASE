// AOJ 2729 Delete Files
// 2018.2.7 bal4u
 
#include <stdio.h>
#include <string.h>

#define QMAX 1002
typedef struct { int k, t; } QUE;
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
		qt = que[i]; que[i] = que[min]; que[min] = qt;
		min_heapify(min);
	}
}

void deq()
{
//	if (qsize == 0) return 0;
//	*a = que[0].a, *b = que[0].b, *c = que[0].c;
	que[0] = que[--qsize];
	min_heapify(0);
}

void enq(int k, int t)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].t = t, que[i].k = k;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i]; que[i] = que[min]; que[min] = qt;
		i = min;
	}
}

char d[1002]; int w[1002];
char mk[1002];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	while (c < '0') c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, i, k, f, len, ans;

    n = in();
	qsize = 0;
	for (i = 0; i < n; i++) {
		f = getchar_unlocked() == 'y', getchar_unlocked();
		d[i] = f, w[i] = in();
		if (f) enq(i, w[i]);
	}

	ans = 0;
	while (qsize) {
		k = que[0].k, len = que[0].t, deq();
		if (mk[k]) continue;

		ans++;
		mk[k] = 1;
		for (i = k-1; i >= 0; i--) {
			if (!d[i] && w[i] >= len) break;
			mk[i] = 1;
		}
		for (i = k+1; i < n; i++) {
			if (!d[i] && w[i] >= len) break;
			mk[i] = 1;
		}
	}
	printf("%d\n", ans);
	return 0;
}
