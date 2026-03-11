// Aizu Vol-15 1500: ID
// 2017.9.4 bal4u@uu

#include <stdio.h>

char id[100003];
char a[10];
int t[10] = { 0,2,4,6,8,1,3,5,7,9 };
long long p[2][10];

int main()
{
	int n, m, i, j, d, x, k, k1, k2;

	scanf("%d%s%d", &n, id+1, &m);
	for (i = 0; i < m; i++) scanf("%d", &d), a[d] = 1;
	p[1][0] = 1;
	for (k1 = 1, k2 = 0, i = 1; i <= n; i++, k1 = k2, k2 = !k2) {
		for (j = 0; j < 10; j++) p[k2][j] = 0;
		if (id[i] == '*') {
			for (k = 0; k < 10; k++) {
				if (!a[k]) continue;
				d = k2 ? t[k] : k;
				for (j = 0; j < 10; j++) {
					if ((x = d + j) >= 10) x -= 10;
					p[k2][x] += p[k1][j];
				}
			}
		} else {
			k = id[i] - '0';
			d = k2 ? t[k] : k;
			for (j = 0; j < 10; j++) {
				if ((x = d + j) >= 10) x -= 10;
				p[k2][x] += p[k1][j];
			}
		}
	}
	printf("%lld\n", p[k1][0]);
	return 0;
}