// AOJ 2162: Galaxy Wide Web Service
// 2017.12.13 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX1 55440

int a[MAX1+1];
int q[25][25];
char f[25], chk[25];
char buf[205], *bp;

int getint(void)
{
	int n = 0;
	while (*bp >= '0') n = (n<<3) + (n<<1) + (*bp++ & 0xf);
	bp++;
	return n;
}

int gcd(int a, int b)
{
	int r;
	while (b != 0) r = a % b, a = b, b = r;
	return a;
}

int lcm(int a, int b)
{
	return a/gcd(a, b)*b;
}

int main()
{
	int n, i, j, jj, k, d, t, p, ans1, ans;

	chk[13] = chk[17] = chk[19] = chk[23] = 1;

	while (fgets(bp=buf, 10, stdin) && *bp != '0') {
		n = getint();

		memset(q, 0, sizeof(q));
		memset(f, 1, sizeof(f));

		for (i = 0; i < n; i++) {
			fgets(bp=buf, 205, stdin);
			d = getint(), t = getint(); f[d] = 0;
			jj = d-t; 			
			j = d; while (j--) {
				if (jj == d) jj = 0;
				q[d][jj++] += getint();
			}
		}

		p = 1;
		for (d = 1; d <= 24; d++) {
			if (f[d]) continue;
			if (!chk[d]) p = lcm(p, d);
		}

		memset(a, 0, sizeof(int)*p);

		ans = 0;
		for (d = 1; d <= 24; d++) {
			if (f[d]) continue;
			if (chk[d]) {
				k = 0;
				for (i = 0; i < d; i++) {
					if (q[d][i] > k) k = q[d][i];
				}
				ans += k;
			} else {
				for (i = 0; i < d; i++) {
					k = q[d][i];
					for (j = i; j < p; j += d) a[j] += k;
				}
			}
		}

		ans1 = 0; for (i = 0; i < p; i++) if (a[i] > ans1) ans1 = a[i];
		printf("%d\n", ans+ans1);
	}
	return 0;
}