// AOJ 1369: Distribution Center
// 2017.11.3 bal4u@uu
// 2018.1.5

#include <stdio.h>
#include <string.h>

int y[100002];
int l[200002], r[200002];

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
	int n, m, i, x, xmin, xmax;

	n = in(), m = in();
	x = in(), y[x] = in();
	xmin = xmax = x;
	i = m-1; while (i--) {
		x = in(), y[x] = in();
		if (x < xmin) xmin = x;
		else if (x > xmax) xmax = x;
	}

	for (i = 1; i <= n; i++) l[i] = i;
	memcpy(r, l, sizeof(int)*(n+1));

	for (x = xmin; x <= xmax; x++) {
		if (y[x]) i = y[x], l[i+1] = l[i], r[i] = r[i+1];
	}
	printf("%d", r[1]-l[1]+1);
	for (i = 2; i <= n; i++) printf(" %d", r[i]-l[i]+1);
	putchar('\n');
	return 0;
}

