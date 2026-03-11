// AOJ 2590: Unknown Switches
// 2017.11.30 bal4u@uu

#include <stdio.h>
#include <string.h>

long long b[1002];
int n;

int bitcount64(long long x)
{
	x = ((x & 0xAAAAAAAAAAAAAALL) >>  1) + (x & 0x55555555555555LL);
	x = ((x & 0xCCCCCCCCCCCCCCLL) >>  2) + (x & 0x33333333333333LL);
	x = ((x & 0xF0F0F0F0F0F0F0LL) >>  4) + (x & 0x0F0F0F0F0F0F0FLL);
	x = ((x & 0x00FF00FF00FF00LL) >>  8) + (x & 0xFF00FF00FF00FFLL);
	x = ((x & 0xFF0000FFFF0000LL) >> 16) + (x & 0x00FFFF0000FFFFLL);
	x = ((x & 0xFFFFFF00000000LL) >> 32) + (x & 0x000000FFFFFFFFLL);
	return (int)x;
}

int tr(long long x)
{
	int k, f = 1;

	while (!(x & 1)) f++, x >>= 1;
	k = n - f;
	if (k < 10) return k + '0';
	return k -10 + 'A';
}

int main()
{
	int m, q, i, j;
	char b1[40], b2[1010], *p;
	long long s, t, mask;

	while (scanf("%d%d%d", &n, &m, &q) && n > 0) {
		memset(b, 0xff, sizeof(b));
		mask = (1 << n) - 1;

		for (s = 0, i = 0; i < q; i++) {
			scanf("%s%s", b1, b2);
			t = 0, p = b1; while (*p >= '0') t = (t<<1) | (*p++ & 1);
			s ^= t;

			for (p = b2, j = 0; j < m; j++) {
				if (*p++ & 1) b[j] &=  s;
				else          b[j] &= ~s;
			}
	    }

		for (i = 0; i < m; i++) {
			if (bitcount64(b[i] & mask) == 1) putchar(tr(b[i]));
			else putchar('?');
		}
		putchar('\n');
	}
	return 0;
}