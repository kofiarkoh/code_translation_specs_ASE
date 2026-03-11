// Aizu 1347: Shopping
// 2017.11.3 bal4u@uu

#include <stdio.h>

int f[1002];

int main()
{
	int n, m, c, d, i, min, max, ans;
	
	scanf("%d%d", &n, &m);
	min = n, max = 0;
	while (m--) {
		scanf("%d%d", &c, &d);
		if (c < min) min = c;
		if (d > max) max = d;
		for (i = c; i < d; i++) f[i] = 1;
	}
	for (ans = 0, i = min; i <= max; i++) ans += f[i];
	printf("%d\n", n + (ans << 1) + 1);
	return 0;
}