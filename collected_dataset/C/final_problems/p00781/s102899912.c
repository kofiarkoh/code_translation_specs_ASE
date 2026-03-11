// AOJ 1201: Lattice Practices
// 2017.12.17 bal4u@uu

#include <stdio.h>
#include <string.h>

char a[10][6], b[10][6];
char *h[5], *v[5];
char diff[10];
int ans;

int check(int j)
{
	int i;
	for (i = 0; i < 5; i++) if (h[i][j] == v[j][i]) return 0;
	return 1;
}

void solve(int k, int s)
{
	int i, j, bit;

	if (k == 10) { ans++; return; }
	for (bit = 1, i = 0; i < 10; i++, bit <<= 1) {
		if (s & bit) continue;
		if (k < 5) {
			h[k] = a[i];
			solve(k+1, s | bit);
			if (diff[i]) {
				h[k] = b[i];
				solve(k+1, s | bit);
			}
		} else {
			j = k - 5;
			v[j] = a[i];
			if (check(j)) solve(k+1, s | bit);
			if (diff[i]) {
				v[j] = b[i];
				if (check(j)) solve(k+1, s | bit);
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
			diff[i] = memcmp(a[i], b[i], 5);
		}
		ans = 0;
		solve(0, 0);
		printf("%d\n", ans >> 3);
	}
	return 0;
}