// AOJ 2162: Galaxy Wide Web Service
// 2017.12.13 bal4u@uu

#include <stdio.h>
#include <string.h>

#define MAX1 55440
#define MAX2 96577

int a[MAX1+1], b[MAX2+1];
char chk[25], f[25];
int p1, p2;

typedef struct { int d, t, q[24]; } T;
T tbl[101];
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
	int n, i, j, jj, k, d, t, q, ans1, ans2;

	chk[13] = chk[17] = chk[19] = chk[23] = 1;

	while (fgets(p=buf, 10, stdin) && *p != '0') {
		n = getint();


		memset(f, 0, sizeof(f));
		p1 = p2 = 1;

		for (i = 0; i < n; i++) {
			fgets(p=buf, 205, stdin);
			d = getint(), t = getint();
			if (chk[d]) {
				if (!f[d]) f[d] = 1, p2 *= d;
			} else p1 = lcm(p1, d);
			
			tbl[i].d = d, tbl[i].t = t;
			for (j = 0; j < d; j++) tbl[i].q[j] = getint();
		}

		memset(a, 0, sizeof(int)*p1);
		memset(b, 0, sizeof(int)*p2);

		for (i = 0; i < n; i++) {
			d = tbl[i].d, t = tbl[i].t;
			if (chk[d]) {
				for (jj = t-1, j = 0; j < d; j++) {
					if (++jj == d) jj = 0;
					q = tbl[i].q[jj];
					for (k = j; k < p2; k += d) b[k] += q;
				}
			} else {
				for (jj = t-1, j = 0; j < d; j++) {
					if (++jj == d) jj = 0;
					q = tbl[i].q[jj];
					for (k = j; k < p1; k += d) a[k] += q;
				}
			}
		}

		ans1 = 0; for (i = 0; i < p1; i++) if (a[i] > ans1) ans1 = a[i];
		ans2 = 0; for (i = 0; i < p2; i++) if (b[i] > ans2) ans2 = b[i];
		printf("%d\n", ans1+ans2);
	}
	return 0;
}