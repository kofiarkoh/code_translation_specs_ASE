// Aizu 1114: Get a Rectangular Field
// 2017.9.13 bal4u@uu

#include <stdio.h>

int map[6][6], sum[6][6];

int main()
{
	int m, r, c, r2, c2, ans;

	scanf("%d", &m);
	while (m-- > 0) {
		for (r = 1; r <= 5; r++) for (c = 1; c <= 5; c++) scanf("%d", &map[r][c]);
		ans = 0;
		for (sum[r][0] = 0, r = 1; r <= 5; r++) for (c = 1; c <= 5; c++) sum[r][c] = sum[r][c-1] + map[r][c];
		for (r = 1; r <= 5; r++) for (c = 1; c <= 5; c++) {
			for (c2 = 5; c2 >= c; c2--) {
				int w, w2;
				if ((w2 = w = c2-c+1) > sum[r][c2]-sum[r][c-1]) continue;
				if (w2 > ans) ans = w2;
				for (r2 = r+1; r2 <= 5; r2++) {
					if (w > sum[r2][c2]-sum[r2][c-1]) break;
					w2 += w; if (w2 > ans) ans = w2;
				}
			}
		}
		printf("%d\n", ans);
	}
	return 0;
}