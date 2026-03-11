// AOJ 2619: Thread Tree
// 2017.11.26 bal4u@uu

#include <stdio.h>
#include <stdlib.h>

typedef struct { int ko, nxt, end; char s[52]; } T;
T node[1002];

void dopr(int k, int level)
{
	int w = level; while (w--) putchar('.');
	puts(node[k].s);
	if (node[k].ko >= 0) dopr(node[k].ko, level+1);
	if (node[k].nxt >= 0) dopr(node[k].nxt, level);
}

int main()
{
	int n, i, k;
	T *p, *q;

	scanf("%d", &n);

	p = node + 1;
	p->ko = p->nxt = p->end = -1;
	scanf("%d%s", &k, p->s);

	for (i = 2; i <= n; i++) {
		q = node + i;
		scanf("%d%s", &k, q->s);
		p = node + k;
		q->ko = q->nxt = q->end = -1;

		if (p->ko < 0) p->ko = p->end = i;
		else {
			node[p->end].nxt = i;
			p->end = i;
		}
	}
	dopr(1, 0);
	return 0;
}