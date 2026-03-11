// AOJ 1275: And Then There Was One
// 2017.10.16 bal4u@uu

#include <stdio.h>

int main()
{
	int n, k, m;
	int a, off;

	while (scanf("%d%d%d", &n, &k, &m) && n > 0) {
		off = (m-k) % n; if (off < 0) off += n;
		for (a = 1; a <= n; a++) m = (m + k) % a;
		printf("%d\n", (m+off+n) % n + 1);
	}
	return 0;
}