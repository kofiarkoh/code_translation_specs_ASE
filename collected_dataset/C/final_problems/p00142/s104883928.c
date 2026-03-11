// AOJ 0142: Nature of Prime Numbers
// 2017.8.16 bal4u@uu
// 2018.1.24

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int a[10002];
int d[10002];
char f[10002];

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int main()
{
	int n, i, j, k, b, m;

	while (n = in()) {
		k = n >> 1;
		memset(d, 0, sizeof(d));
		memset(f, 0, sizeof(f));
		for (m = 0, i = 1; i <= k; i++) {
			b = i*i% n;
			if (!f[b]) f[b] = 1, a[m++] = b;
		}
		for (i = 0; i < m; i++) {
			for (j = 0; j < i; j++) {
				if (b = a[i] - a[j]) {
					if (b < 0) b += n;
					if (b > k) b = n - b;
					d[b]++;
				}
			}
		}
		for (i = 1; i <= k; i++) printf("%d\n", d[i]<<1);
	}
	return 0;
}

