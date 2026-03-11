// Aizu Vol-1 0142: Nature of Prime Numbers
// 2017.8.16 bal4u@uu
// 4m+3????´???°?????§???

#include <stdio.h>
#include <string.h>

#define MAX  10000
#define SQRT 100     // sqrt(MAX)
#define HALF 5000
int sqr[HALF + 5];
int a[HALF + 5];
int d[HALF + 5];
char f[MAX + 5];

char prime[MAX + 5], *p, *q, *pmax;

int main()
{
	int n;
	int i, j, k, b, m;

	pmax = prime + MAX;
	for (k = 3, p = prime + 3; k <= SQRT; k += 2, p += 2) {
		if (!*p) {
			for (q = p + k; q <= pmax; q += k) *q = 1;
		}
	}
	for (i = 1; i <= 5000; i++) sqr[i] = i*i;

	while (1) {
		scanf("%d", &n); if (!n) break;
		if (n == 1 && !(n & 1)) while (1);		// Panic!

		// 4m+3????´???°
		if (!prime[n] && n % 4 == 3) {	
			char buf[10];
			k = n >> 1;
			sprintf(buf, "%d", k - 1);
			for (i = 1; i <= k; i++) puts(buf);
			continue;
		}

		// ???????\???°????????????????????????????¨????
		k = n >> 1;
		memset(d, 0, sizeof(d[0])*(k+2));
		memset(f, 0, sizeof(f[0])*(n+2));
		m = 0;
		for (i = 1; i <= k; i++) {
			b = sqr[i] % n;
			if (!f[b]) f[b] = 1, a[m++] = b;
		}

		for (i = 0; i < m; i++) {
			for (j = 0; j < m; j++) {
				if (b = a[i] - a[j]) {
					if (b < 0) b += n;
					if (b > k) b = n - b;
					d[b]++;
				}
			}
		}
		for (i = 1; i <= k; i++) printf("%d\n", d[i]);
	}
	return 0;
}