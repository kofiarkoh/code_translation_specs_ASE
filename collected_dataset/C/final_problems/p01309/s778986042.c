// AOJ 2191: A Book Shop With a Frequent Greetings
// 2017.12.10

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define QSIZE 2000000
#define PARENT(i) ((i)>>1)
#define LEFT(i)   ((i)<<1)
#define RIGHT(i)  (((i)<<1)+1)

typedef struct { int t, n; } QUE;
QUE que[QSIZE+2];
int qsize;

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
	que[0] = que[--qsize];
	min_heapify(0);
}

int max;
void enq(int n, int t)
{
	int i, min;
	QUE qt;

	i = qsize++;
	que[i].t = t, que[i].n = n;
	while (i > 0 && que[min = PARENT(i)].t > que[i].t) {
		qt = que[i]; que[i] = que[min]; que[min] = qt;
		i = min;
	}
}

int xp[1001], yp[1001];
int rel[1001][1001];
int len[1001];
int prev[1001];

char buf[30], *p;
int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	p++;
	return n;
}

int main()
{
	int cno, n, X, Y, x0, y0, i, j, t, ans;

	fgets(p=buf, 10, stdin), cno = getint();
	while (cno--) {
		fgets(p=buf, 30, stdin);
		n = getint(), X = getint(), Y = getint();
		fgets(p=buf, 30, stdin);
		x0 = getint(), y0 = getint();
		for (i = 0; i < n; i++) {
			fgets(p=buf, 30, stdin);
			xp[i] = getint(), yp[i] = getint();
		}

		ans = 0, qsize = 0;
		memset(prev, 0x81, sizeof(prev));
		memset(len, 0, sizeof(len));
		for (i = 0; i < n; i++) {
			if (hypot(xp[i]-x0, yp[i]-y0) <= 10) enq(i, 0);
			for (j = i+1; j < n; j++) {
				if (hypot(xp[i]-xp[j], yp[i]-yp[j]) <= 50)
					rel[i][len[i]++] = j, rel[j][len[j]++] = i;
			}
		}

		while (qsize) {
			i = que[0].n, t = que[0].t, deq();
			if (t > n*X) { ans = -1; break; }
			if (prev[i] < t-Y) {
				t += X;
				prev[i] = t;
				if (t > ans) ans = t;
				for (j = 0; j < len[i]; j++) enq(rel[i][j], t);
			}
		}
		if (ans < 0) puts("You're always welcome!");
		else printf("%d\n", ans);
	}
	return 0;
}