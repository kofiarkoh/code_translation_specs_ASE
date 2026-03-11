#include <stdio.h>
#include <stdlib.h>

#define N 100000
#define A 100000

int aa[N], kk[A + 1];

int compare(const void *a, const void *b) {
	int i = *(int *) a;
	int j = *(int *) b;

	return kk[aa[i]] != kk[aa[j]] ? kk[aa[j]] - kk[aa[i]] : i - j;
}

int good(int n) {
	int i;

	for (i = 0; i < n; i++)
		if (aa[i] != -1 && kk[aa[i]] > 1)
			return 0;
	return 1;
}

int main() {
	static int ii[N];
	int n, i, a, b, c, x, y, k;

	scanf("%d", &n);
	for (i = 0; i < n; i++) {
		scanf("%d", &aa[i]);
		kk[aa[i]]++;
		ii[i] = i;
	}
	qsort(ii, n, sizeof *ii, compare);
	for (i = 0; i + 2 < n && !good(n); i += 2) {
		a = aa[ii[i]];
		b = aa[ii[i + 1]];
		c = aa[ii[i + 2]];
		if (i + 3 < n && a != b)
			i++;
		if (i + 4 < n && b != a && b != c)
			i++;
		if (i + 5 < n && c != b)
			i++;
		a = aa[ii[i]];
		b = aa[ii[i + 1]];
		c = aa[ii[i + 2]];
		x = y = a;
		if (x < b)
			x = b;
		if (y > b)
			y = b;
		if (x < c)
			x = c;
		if (y > c)
			y = c;
		if ((x == a && y == b) || (x == b && y == a))
			aa[ii[i]] = aa[ii[i + 1]] = -1;
		else if ((x == a && y == c) || (x == c && y == a))
			aa[ii[i]] = aa[ii[i + 1]] = -1, aa[ii[i + 2]] = b;
		else
			aa[ii[i]] = aa[ii[i + 1]] = -1, aa[ii[i + 2]] = a;
		kk[x]--;
		kk[y]--;
	}
	k = 0;
	for (i = 0; i < n; i++)
		if (aa[ii[i]] != -1)
			k++;
	printf("%d\n", k);
	return 0;
}