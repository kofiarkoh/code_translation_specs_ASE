// AOJ 1575: Product Sale Lines
// 2017.9.29 bal4u@uu

#include <stdio.h>

int main()
{
	int w, h, n, p, x, y, ans;

	scanf("%d%d%d", &w, &h, &n);
	ans = x = y = 0;
	while (n--) {
		scanf("%d", &p);
		if (!p) x++; else y++;
		if (x < w-1) {
			if (y == x) ans++;
		} else if (x == w-1) ;
		else if (x < w+h-2) {
			if (y+2 == x) ans++;
		} else if (x == w+h-2) ;
		else {
			if (y+4 == x) ans++;
		}
	}
	printf("%d\n", ans);
	return 0;
}