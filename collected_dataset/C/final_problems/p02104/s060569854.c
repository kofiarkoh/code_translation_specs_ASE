// AOJ 3006: Chairs
// 2017.10.4 bal4u@uu

#include <stdio.h>

#define MAX		100003
int p[MAX];
int spool[MAX], *pt;
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
	int n, i, j;

	n = in();
	for (i = 1; i <= n; i++) p[i] = j = in(), hh[j]++;

	pt = spool;
	for (i = 1; i <= n; i++) q[i] = pt, pt += hh[i];

	for (i = 1; i <= n; i++) j = p[i], q[j][hi[j]++] = i;

	top = 0;
	j = 2; while (j--) {
		for (i = 1; i <= n; i++) {
			while (hi[i]) sp[top++] = q[i][--hi[i]];
			if (!ans[i] && top) ans[i] = sp[--top];
		}
    }
 
	for (i = 1; i <= n; i++) printf("%d\n", ans[i]);
	return 0;
}
