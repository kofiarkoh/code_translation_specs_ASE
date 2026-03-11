// Aizu 3014: Hating Crowd
// 2017.9.25 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MIN(a,b) ((a)<=(b)?(a):(b))
#define MAX(a,b) ((a)>=(b)?(a):(b))
#define ABS(a)   ((a)>=0?(a):-(a))

typedef struct { int d, e, s; } T;
T tbl[102];

int main()
{
	int n, i, m, d, v, x, k, max, ans;

	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d%d%d%d", &m, &d, &v, &tbl[i].s);
		tbl[i].d = (m-1)*30 + d;
		tbl[i].e = tbl[i].d + v-1;
	}
	ans = 0x7ffffff;
	for (x = 1; x <= 360; x++) {
		max = 0;
		for (i = 0; i < n; i++) {
			if (tbl[i].e > 360) {
				if (tbl[i].d <= x && x <= 360) k = tbl[i].s;
				else if (1 <= x && x <= tbl[i].e - 360) k = tbl[i].s;
				else k = MAX(0, tbl[i].s - MIN(ABS(tbl[i].d - x), ABS(x - (tbl[i].e-360))));
			} else {
				if (tbl[i].d <= x && x <= tbl[i].e) k = tbl[i].s;
				else {
					int min = 0x7ffffff, a;
					a = ABS(tbl[i].d - x);  if (a < min) min = a;
					a = ABS(tbl[i].e - x);  if (a < min) min = a;
					a = tbl[i].d + 360 - x; if (a < min) min = a;
					a = x + 360 - tbl[i].e; if (a < min) min = a;
					k = MAX(0, tbl[i].s - min);
				}
			}
			if (k > max) max = k;
		}
		if (max < ans) ans = max;
	}
	printf("%d\n", ans);
	return 0;
}