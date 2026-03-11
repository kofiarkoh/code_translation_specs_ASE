// AOJ 1502: War
// 2017.11.17 bal4u@uu

#include <stdio.h>

int h[126];

int main()
{
	int n, i, k, v, s, ans;

	scanf("%d", &n);
	for (ans = 1, i = 0; i < n; i++) {
		scanf("%d", &v);
		if (v <= 125) h[v]++;
		ans += v;
	}

	for (s = 0, k = 4, i = 1; i <= 125; i++, k += 4) {
		s += h[i-1];
		if (n > s+k) ans -= n-(s+k);
	}
	printf("%d\n", ans);
	return 0;
}