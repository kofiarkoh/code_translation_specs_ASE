// AOJ 2176: For the Peace
// 2017.12.15 bal4u@uu

#include <stdio.h>
#include <string.h>

#define ABS(a)  ((a)>=0?(a):-(a))

int pool[10003], sz;
int *c[101], m[101], s[101];
char buf[50003], *p;

int getint()
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	p++;
	return n;
}

int main()
{
	int n, d, i, j, k, max, max2, f, g;

	while (fgets(p=buf, 15, stdin) && *p != '0') {
		n = getint(), d = getint();

		sz = 0;
		for (i = 0; i < n; i++) {
			fgets(p=buf, sizeof(buf), stdin);
			m[i] = getint(), s[i] = 0;
			c[i] = pool + sz;
			for (j = 0; j < m[i]; j++) c[i][j] = k = getint(), s[i] += k;
			sz += m[i];
		}

		while (sz) {
			max = max2 = 0;
			for (i = 0; i < n; i++) {
				if (s[i] >= max) max2 = max, max = s[i];
				else if (s[i] > max2) max2 = s[i];
			}

			f = 0;
			for (i = 0; i < n; i++) {
				while (m[i] > 0) {
					g = c[i][m[i]-1];
					if (max - (s[i] - g) > d) break;
					m[i]--, s[i] -= g, f = 1, sz--;
				}
			}
			if (f) continue;

			for (i = 0; i < n; i++) {
				if (s[i] == max) {
					while (m[i] > 0) {
						g = c[i][m[i]-1];
						if (ABS(s[i] - g - max2) > d) break;
						m[i]--, s[i] -= g, f = 1, sz--;
					}
				}
			}
			if (!f) break;
		}
		puts(sz ? "No" : "Yes"); 
	}
	return 0;
}