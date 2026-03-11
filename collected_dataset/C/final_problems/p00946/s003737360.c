// AOJ 1367: Rearranging a Sequence
// 2017.10.9 bal4u@uu
// 2018.1.8

#include <stdio.h>

typedef struct { int id, a; } T;
T tbl[200002], tb2[200002];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

#define VMAX 300000
int f[VMAX+1];
void distsort(int n)
{
	int i, x;
	T *tp;

//	memset(f, 0, sizeof(f));
	tp = tbl+1, i = n; while (i--) f[tp->id]++, tp++;
	for (i = 2; i <= VMAX; i++) f[i] += f[i-1];
	for (i = n; i > 0; i--) x = tbl[i].id, tb2[--f[x]] = tbl[i];
}

int main()
{
	int n, m, i, j;
	register T *tp;

	n = in(), m = in();
	tp = tbl+1, j = n; for (i = 1; i <= n; i++) tp->id = j--, tp->a = i, tp++;

	while (m-- > 0) tbl[in()].id = i++;

	distsort(n);
	while (n--) printf("%d\n", tb2[n].a);
	return 0;
}

