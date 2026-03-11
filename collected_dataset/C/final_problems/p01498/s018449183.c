// AOJ 2382: King Slime
// 2018.1.22 bal4u@uu

#include <stdio.h>

#define MAX 40002

/* UNION-FIND library */
int id[MAX], size[MAX];

void init(int n)
{
	int i;
	for (i = 1; i <= n; i++) id[i] = i, size[i] = 1;
}

int root(int i)
{
	while (i != id[i]) id[i] = id[id[i]], i = id[i];
	return i;
}

int connected(int p, int q) { return root(p) == root(q); }

void unite(int p, int q)
{
    int i = root(p), j = root(q);
	if (i == j) return;
    if (size[i] < size[j]) id[i] = j, size[j] += size[i];
    else                   id[j] = i, size[i] += size[j];
}
/* UNION-FIND library */


int xx[100002], yy[100002];
char tbl[40002];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, w, h, i, x, y, f, ans;

	n = in(), w = in(), h = in();
	init(n);

	f = 0;
	for (i = 1; i <= n; i++) {
		x = in(), y = in();
		if (x == 1 || x == w || y == 1 || y == h) f = 1;
		if (!xx[x]) xx[x] = i; else unite(xx[x], i);
		if (!yy[y]) yy[y] = i; else unite(yy[y], i);
	}

	ans = 0;
	for (i = 1; i <= n; i++) {
		x = root(i);
		if (!tbl[x]) ans++, tbl[x] = 1;
	}
	if (ans == 1) f = 1;
	ans += n - (f + 1);
	printf("%d\n", ans);
	return 0;
}
