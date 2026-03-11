// AOJ 2590: Unknown Switches
// 2017.11.30 bal4u@uu

#include <stdio.h>
#include <string.h>

long long b[1002];
int n;
char buf[1100], *p;

int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	return n;
}

int check(long long x)
{
	int k, f = 1;

	if (x == 0) return -1;
	while (!(x & 1)) f++, x >>= 1;
	x >>= 1; if (x) return -1;
	k = n - f;
	if (k < 10) return k + '0';
	return k - 10 + 'A';
}

int main()
{
	int m, q, i, j, c;
	long long s, t, mask;

	while (fgets(p=buf, 30, stdin) && *p != '0') {
		n = getint(), p++, m = getint(), p++, q = getint();

		memset(b, 0xff, sizeof(b));
		mask = (1LL << n) - 1;

		for (s = 0, i = 0; i < q; i++) {
			fgets(p=buf, 1100, stdin);
			t = 0; while (*p >= '0') t = (t<<1) | (*p++ & 1);
			s = s ^ t;

			for (p++, j = 0; j < m; j++) {
				if (*p++ & 1) b[j] &=  s;
				else          b[j] &= ~s;
			}
	    }

		for (i = 0; i < m; i++) {
			if ((c = check(b[i] & mask)) > 0) putchar(c);
			else putchar('?');
		}
		putchar('\n');
	}
	return 0;
}