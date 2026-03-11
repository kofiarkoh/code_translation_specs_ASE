// Aizu 2049: Headstrong Student
// 2017.9.24 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX 1000000
int tbl[MAX+2];

char buf[20], *bp;
int getint()
{
	int n = 0;
	while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf);
	return n;
}

int main()
{
	int x, y, p, q, r;

	while (fgets(bp=buf, 20, stdin) && *buf != '0') {
		x = getint(), bp++, y = getint();
		memset(tbl, -1, sizeof(tbl));
		tbl[x] = 0, p = q = 0, r = 1;
		while (1) {
			x = ((x<<3)+(x<<1)) % y;
			if (!x) { p = r; break; }
			if (tbl[x] == -1) tbl[x] = r++;
			else { p = tbl[x], q = r - tbl[x]; break; }
		}
		printf("%d %d\n", p, q);
	}
	return 0;
}