#include <stdio.h>
#include <string.h>

#define N	500000

int max(int a, int b) { return a > b ? a : b; }

void zzz(char *cc, int *zz, int n) {
	int i, l, r;

	for (i = 1, l = r = 0; i < n; i++)
		if (zz[i - l] < r - i)
			zz[i] = zz[i - l];
		else {
			l = i, r = max(r, i);
			while (r < n && cc[r] == cc[r - l])
				r++;
			zz[i] = r - l;
		}
}

int main() {
	static char cc[N + 1], cc_[N + 1], bad[N + 1];
	static int zz[N], zz_[N];
	int n, p, m, i, cnt;

	scanf("%s", cc), n = strlen(cc);
	zzz(cc, zz, n);
	for (p = 1; p < n; p++)
		if (n % p == 0 && zz[p] == n - p)
			break;
	if (p == n) {
		printf("1\n");
		printf("1\n");
		return 0;
	}
	if (p == 1) {
		printf("%d\n", n);
		printf("1\n");
		return 0;
	}
	for (i = 0; i < n; i++)
		cc_[i] = cc[n - 1 - i];
	zzz(cc_, zz_, n);
	for (p = 1; p < n; p++) {
		for (m = p + p; m <= n && zz[p] >= m - p; m += p)
			bad[m] = 1;
		for (m = p + p; m <= n && zz_[p] >= m - p; m += p)
			bad[n - m] = 1;
	}
	cnt = 0;
	for (m = 1; m < n; m++)
		if (!bad[m])
			cnt++;
	printf("2\n");
	printf("%d\n", cnt);
	return 0;
}
