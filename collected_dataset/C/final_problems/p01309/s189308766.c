// AOJ 2191: A Book Shop With a Frequent Greetings
// 2017.12.10

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define QSIZE 1000000
typedef struct { int n, t; } Q;
Q q[QSIZE]; int top, end;

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

		ans = 0, top = end = 0;
		memset(prev, 0x81, sizeof(prev));
		memset(len, 0, sizeof(len));
		for (i = 0; i < n; i++) {
			if (hypot(xp[i]-x0, yp[i]-y0) <= 10) q[end].n = i, q[end++].t = 0;
			for (j = i+1; j < n; j++) {
				if (hypot(xp[i]-xp[j], yp[i]-yp[j]) <= 50)
					rel[i][len[i]++] = j, rel[j][len[j]++] = i;
			}
		}

		while (top < end) {
			i = q[top].n, t = q[top++].t;
			if (X > Y && t > X) { ans = -1; break; }
			if (prev[i] < t-Y) {
				t += X;
				prev[i] = t;
				if (t > ans) ans = t;
				for (j = 0; j < len[i]; j++) q[end].n = rel[i][j], q[end++].t = t;
			}
		}
		if (ans < 0) puts("You're always welcome!");
		else printf("%d\n", ans);
	}
	return 0;
}