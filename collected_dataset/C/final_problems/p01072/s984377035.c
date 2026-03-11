// AOJ 1587: Plants
// 2017.10.23 bal4u@uu

#include <stdio.h>
#include <string.h>

int hi[51][51];

int main() 
{
	int w, h, t, p, x, y, v, ans;

	scanf("%d%d%d%d", &w, &h, &t, &p);
	while (p-- > 0) {
		scanf("%d%d%d", &x, &y, &v);
		hi[x][y]++;
	}
	ans = 0;
	for (y = 0; y < h; y++) for (x = 0; x < w; x++) {
		scanf("%d", &v);
		if (v) ans += hi[x][y];
	}
	printf("%d\n", ans);
	return 0; 
}