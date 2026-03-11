// Aizu 2100: Saizo
// 2017.9.20 bal4u@uu

#include <stdio.h>

int main()
{
	int t, n, h, h0, d, up, dn;

	scanf("%d", &t);
	while (t-- > 0) {
		scanf("%d%d", &n, &h0), up = dn = 0;
		while (n-- > 1) {
			scanf("%d", &h);
			if ((d = h - h0) > 0) { if (d > up) up = d; }
			else          { d = -d; if (d > dn) dn = d; }
			h0 = h;
		}
		printf("%d %d\n", up, dn);
	}
	return 0;
}