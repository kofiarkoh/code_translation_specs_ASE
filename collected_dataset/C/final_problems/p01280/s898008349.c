// AOJ 2162: Galaxy Wide Web Service
// 2017.12.13 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX1 55440
#define MAX2 96577

int a[MAX1+1], b[MAX2+1];
int p1, p2;
int q[25][25];
char f[25], chk[25];
char buf[205], *p;

int getint(void)
{
	int n = 0;
	while (*p >= '0') n = (n<<3) + (n<<1) + (*p++ & 0xf);
	p++;
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
	int n, i, j, jj, k, d, t, ans1, ans2;

	chk[13] = chk[17] = chk[19] = chk[23] = 1;

	while (fgets(p=buf, 10, stdin) && *p != '0') {
		n = getint();

		memset(q, 0, sizeof(q));
		memset(f, 1, sizeof(f));

		for (i = 0; i < n; i++) {
			fgets(p=buf, 205, stdin);
			d = getint(), t = getint(); f[d] = 0;
			jj = 0; if (t > 0) jj = d-t; 			
			j = d; while (j--) {
				q[d][jj] += getint();
				if (++jj == d) jj = 0;
			}
		}

		p1 = p2 = 1;
		for (d = 1; d <= 24; d++) {
			if (f[d]) continue;
			if (chk[d]) p2 *= d;
			else p1 = lcm(p1, d);
		}

		memset(a, 0, sizeof(int)*p1);
		memset(b, 0, sizeof(int)*p2);

		for (d = 1; d <= 24; d++) {
			if (f[d]) continue;
			if (chk[d]) {
				for (i = 0; i < d; i++) {
					k = q[d][i];
					for (j = i; j < p2; j += d) b[j] += k;
				}
			} else {
				for (i = 0; i < d; i++) {
					k = q[d][i];
					for (j = i; j < p1; j += d) a[j] += k;
				}
			}
		}

		ans1 = 0; for (i = 0; i < p1; i++) if (a[i] > ans1) ans1 = a[i];
		ans2 = 0; for (i = 0; i < p2; i++) if (b[i] > ans2) ans2 = b[i];
		printf("%d\n", ans1+ans2);
	}
	return 0;
}