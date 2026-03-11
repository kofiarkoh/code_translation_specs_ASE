// Aizu 1117: Missing Numbers
// 2017.9.15 bal4u@uu

#include <stdio.h>
#include <stdlib.h>

typedef struct { int r, c, f; } T;
T q[1005]; int len;
long long a[105][15]; char b[105][15]; int h, w;
long long sr[105], sc[15];
int kr[105], kc[15];

int main()
{
	int r, c, i, k, f;
	char buf[20];
	int cno = 0;

	while (scanf("%d", &h) && h) {
		scanf("%d", &w); cno++;
		for (len = 0, r = 0; r <= h; r++) for (c = 0; c <= w; c++) {
			scanf("%s", buf);
			if (*buf == '?') a[r][c] = 0, b[r][c] = 1, q[len].r = r, q[len].c = c, q[len++].f = 1;
			else             a[r][c] = atoi(buf), b[r][c] = 0;
		}
		f = 1; while (f) {
			f = 0;
			for (r = 0; r < h; r++) {
				for (sc[r] = k = 0, c = 0; c < w; c++) {
					sc[r] += a[r][c], k += b[r][c]; if (b[r][c]) i = c;
				}
				if (k == 1) {
					c = i, a[r][c] = a[r][w] - sc[r], b[r][c] = 0, sc[r] += a[r][c], sr[c] += a[r][c];
					f = 1;
				}
			}
			for (c = 0; c < w; c++) {
				for (sr[c] = k = 0, r = 0; r < h; r++) {
					sr[c] += a[r][c], k += b[r][c]; if (b[r][c]) i = r;
				}
				if (k == 1) {
					r = i, a[r][c] = a[h][c] - sr[c], b[r][c] = 0, sr[c] += a[r][c], sc[r] += a[r][c];
					f = 1;
				}
			}
		}
		for (k = 0, i = 0; i < len; i++) if (b[q[i].r][q[i].c]) k = 1; else q[i].f = 0;
		if (cno > 1) putchar('\n');
		if (k == 0) for (i = 0; i < len; i++) printf("%lld\n", a[q[i].r][q[i].c]);
		else puts("NO");
    }
	return 0;
}