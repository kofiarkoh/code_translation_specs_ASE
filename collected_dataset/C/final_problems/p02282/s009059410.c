// AOJ ALDS1_7_D Reconstruction of a Tree
// 2018.5.7 bal4u

#include <stdio.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int pre[45], ind[45];
int post[45], sz, pos;
int idx[45];

void rebuild(int l, int r)
{
	int c, m;

	if (l >= r) return;
	c = pre[pos++];
	m = idx[c];
	rebuild(l, m);
	rebuild(m+1, r);
	post[sz++] = c;
}

int main()
{
	int n, i;

	n = in();
	for (i = 0; i < n; i++) pre[i] = in();
	for (i = 0; i < n; i++) idx[in()] = i;

	sz = 0, pos = 0;
	rebuild(0, n);
	
	printf("%d", post[0]);
	for (i = 1; i < n; i++) {
		putchar(' '), printf("%d", post[i]);
	}
	putchar('\n');
	return 0;
}
