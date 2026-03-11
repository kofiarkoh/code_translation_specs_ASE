// AOJ 2142: Bitwise Kingdom
// 2017.11.11 bal4u@uu

#include <stdio.h>

long long c[62][62], s[62][62];

int bsch(int n, long long x)
{
	int m, l = 0, r = n+1;

    while (l < r) {
        m = (l + r) >> 1;
        if (s[n][m] < x) l = m + 1; else r = m;
    }
	return l-1;
}

int main()  
{  
	int n, i, j;
	long long m;

	for (c[0][0] = 1, i = 1; i <= 60; i++) {
		s[i][0] = c[i][0] = c[i][i] = 1;
		for (j = 1; j < i; j++) {
			c[i][j] = c[i-1][j-1] + c[i-1][j];
			s[i][j] = s[i][j-1] + c[i][j];
		}
		s[i][i] = s[i][i-1] + 1;
	}

	while (scanf("%d%lld", &n, &m) && n > 0) {
		j = bsch(n, m);
		m -= s[n][j++];
		for (i = 1; i <= n; i++) {
			if (j == 0 || c[n-i][j] >= m) putchar('0');
			else {
				putchar('1');
				m -= c[n-i][j--];
			}
		}
		putchar('\n');
	}
	return 0;
}