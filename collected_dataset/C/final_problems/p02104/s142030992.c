// AOJ 3006: Chairs
// 2017.10.4 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX		100003
int p[MAX];
int spool[MAX];
int *q[MAX], hi[MAX], hh[MAX];
int sp[MAX], top;
int ans[MAX];

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
	int n, a, i;
	int *pt;

	n = in();
	for (i = 1; i <= n; i++) {
		p[i] = a = in();
		hh[a]++;
	}
	pt = spool;
	for (i = 1; i <= n; i++) q[i] = pt, pt += hh[i];
	for (i = 1; i <= n; i++) a = p[i], q[a][hi[a]++] = i;

	top = 0, a = 0;
	while (1) for (i = 1; i <= n; i++) {
		while (hi[i]) sp[top++] = q[i][--hi[i]];
		if (!ans[i] && top) {
			ans[i] = sp[--top];
			if (++a == n) goto done;
		}
    }
done:   
	for (i = 1; i <= n; i++) printf("%d\n", ans[i]);
	return 0;
}
