// Aizu 1119: Exploring Caves
// 2017.9.15 bal4u@uu

#include <stdio.h>

int main()
{
	int n, dx, dy, d, x, y, tx, ty, ans;

	scanf("%d", &n);
	while (n-- > 0) {
		ans = 0, x = y = 0, tx = ty = 0;
		while (1) {
			scanf("%d%d", &dx, &dy); if (!dx && !dy) break;
			x += dx, y += dy;
			if ((d = x*x+y*y) > ans) ans = d, tx = x, ty = y;
			else if (d == ans && tx < x) tx = x, ty = y;
		}
		printf("%d %d\n", tx, ty);
	}
	return 0;
}