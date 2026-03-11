// AOJ 1369: Distribution Center
// 2017.11.3 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct { int x, y; } PP;
PP p[100002], p2[100002];
int l[200002], r[200002]; int m;

char buf[20], *bp;
int getint()
{
	int n = 0;
	while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf);
	return n;
}

#define VMAX 100002
int f[VMAX+1], vmax, vmin;
void distsort()
{
	int i, x;
	PP *pt, *end = p+m;

	memset(f, 0, sizeof(int)*(vmax+1));
	for (pt = p; pt < end; pt++) f[pt->x]++;
	for (i = vmin+1; i <= vmax; i++) f[i] += f[i-1];
	for (i = m-1; i >= 0; i--) x = p[i].x, p2[--f[x]] = p[i];
}

int main()
{
	int n, i, x;
	PP *pt;

	fgets(bp=buf, 20, stdin);
	n = getint(), bp++, m = getint();
	vmin = 100001, vmax = 0;
	pt = p; i = m; while (i--) {
		fgets(bp=buf, 20, stdin);
		pt->x = x = getint(), bp++, pt->y = getint(), pt++;
		if (x < vmin) vmin = x;
		if (x > vmax) vmax = x;
	}
	distsort();

	for (i = 1; i <= n; i++) l[i] = i;
	memcpy(r, l, sizeof(int)*(n+1));

	pt = p2; while (m--) {
		i = pt->y, pt++;
		l[i+1] = l[i], r[i] = r[i+1];
	}
	printf("%d", r[1]-l[1]+1);
	for (i = 2; i <= n; i++) printf(" %d", r[i]-l[i]+1);
	putchar('\n');
	return 0;
}