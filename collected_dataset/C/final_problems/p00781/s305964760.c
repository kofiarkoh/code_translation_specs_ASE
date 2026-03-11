// AOJ 1201: Lattice Practices
// 2017.12.17 bal4u@uu

#include <stdio.h>
#include <string.h>

char a[10][6], b[10][6];
char *h[5], *v[5];
int diff[10];
int ans;

int check(int j)
{
	int i, k;
	for (k = 0; k <= j; k++) {
		for (i = 0; i <= j; i++) if (h[i][k] == v[k][i]) return 0;
	}
	return 1;
}

void solve(int k, int s)
{
	int i, x, y, bit;

	if (k == 10) { ans++; return; }
	x = k & 1, y = k >> 1;
	for (bit = 1, i = 0; i < 10; i++, bit <<= 1) {
		if (s & bit) continue;
		if (!x) {
			h[y] = a[i];
			solve(k+1, s | bit);
			if (diff[i]) {
				h[y] = b[i];
				solve(k+1, s | bit);
			}
		} else {
			v[y] = a[i];
			if (check(y)) solve(k+1, s | bit);
			if (diff[i]) {
				v[y] = b[i];
				if (check(y)) solve(k+1, s | bit);
			}
		}
	}
}

int main()
{
	int i, j;

	while(scanf("%s", a[0]) && a[0][0] != 'E') {
		for (i = 1; i < 10; i++) scanf("%s", a[i]);
		for (i = 0; i < 10; i++) {
			for (j = 0; j < 5; j++) b[i][j] = a[i][4-j];
			diff[i] = (memcmp(a[i], b[i], 5) != 0);
		}
		ans = 0;
		solve(0, 0);
		printf("%d\n", ans >> 3);
	}
	return 0;
}