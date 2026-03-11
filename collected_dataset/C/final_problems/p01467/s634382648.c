// Aizu 2350: A-B Problem
// 2017.9.27 bal4u@uu

#include <stdio.h>
#include <string.h>

char a[15], b[15], c[15];
char f[15];					// ??°???????????????????????????????????????
int len;

int getint(char *b, int w)
{
	int n = 0;
	while (w-- > 0) n = 10 * n + (*b++);
	return n;
}

int main()
{
	int i, j, k, m, bw, ans;
	char tmp[15];

	scanf("%s%s%d", a, tmp, &k);
	len = strlen(a); for (i = 0; i < len; i++) a[i] -= '0';
	m = strlen(tmp); for (i = 0; i < m; i++) tmp[i] -= '0';
	memset(b, 0, sizeof(b));
	for (i = 0; i < m; i++) b[i+len-m] = tmp[i];

	memset(f, 0, sizeof(f));
	for (bw = 0, j = len - 1; j >= 0; j--) {
		if (a[j] - bw >= b[j]) bw = 0;
		else bw = 1, f[j-1] = 1;
	}

	ans = 0;
	memset(c, 0, sizeof(c));
	for (m = 0, j = 0; m < k && j < len; j++) {
		if (j == 0) { c[j] = a[j] - b[j]; if (f[0]) m++; }
		else if (f[j-1]) {
			int m1, m2;
			m1 = (10 + a[j] - b[j]) % 10;
			m2 = 9 + a[j] - b[j];
			if (m1 > m2) c[j] = m1, m++;
			else         c[j] = m2;
		} else c[j] = a[j] - b[j];
	}

	for (bw = 0, i = j, j = len - 1; j >= i; j--) {
		if (a[j] - bw >= b[j]) c[j] = a[j] - bw - b[j], bw = 0;
		else c[j] = a[j] - bw + 10 - b[j], bw = 1;
	}

	printf("%d\n", getint(c, len));
	return 0;
}