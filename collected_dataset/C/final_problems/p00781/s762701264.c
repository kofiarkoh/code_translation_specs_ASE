// AOJ 1201: Lattice Practices
// 2017.12.17 bal4u@uu

#include <stdio.h>
#include <string.h>

char a[10][6], b[10][6];
char *h[5], *v[5];
int same[10];
int ans;
void vset(int k, int s);

int check(int j)
{
	int i, k;
	for (i = 0; i <= j; i++) for (k = 0; k <= j; k++) if (h[i][k] == v[k][i]) return 0;
	return 1;
}

void hset(int k, int s)
{
	int i, bit;

	if (k == 5) { ans++; return; }
	for (bit = 1, i = 0; i < 10; i++, bit <<= 1) {
		if (s & bit) continue;
		h[k] = a[i];
		vset(k, s | bit);
		if (same[i]) continue;
		h[k] = b[i];
		vset(k, s | bit);
	}
}

void vset(int k, int s)
{
	int i, bit;

	for (bit = 1, i = 0; i < 10; i++, bit <<= 1) {
		if (s & bit) continue;
		v[k] = a[i];
		if (check(k)) hset(k+1, s | bit);
		if (same[i]) continue;
		v[k] = b[i];
		if (check(k)) hset(k+1, s | bit);
	}
}

int main()
{
	int i, j;

	while(scanf("%s", a[0]) && a[0][0] != 'E') {
		for (i = 1; i < 10; i++) scanf("%s", a[i]);
		for (i = 0; i < 10; i++) {
			for (j = 0; j < 5; j++) b[i][j] = a[i][4-j];
			same[i] = (memcmp(a[i], b[i], 5) == 0);
		}
		ans = 0;
		hset(0, 0);
		printf("%d\n", ans >> 3);
	}
	return 0;
}