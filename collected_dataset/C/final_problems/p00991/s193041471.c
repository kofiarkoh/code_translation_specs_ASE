// AOJ 1501: Grid
// 2017.11.17 bal4u@uu

#include <stdio.h>

#define M 100000007
int tbl[1001][1001];

int combi(int n, int k)
{
	int ans;

	if (tbl[n][k]) return tbl[n][k];
	if ((k << 1) > n) k = n-k;
	if (k == 0) ans = 1;
	else if (k == 1) ans = n;
	else ans = combi(n-1, k) + combi(n-1, k-1);
	return tbl[n][k] = ans % M;
}

int main()
{
	int r, c, a1, b1, a2, b2, dr, dc, k, m;

	k = 0;
	scanf("%d%d%d%d%d%d", &r, &c, &a1, &a2, &b1, &b2);

	dr = a1-b1;
	if (dr < 0) dr = -dr;
	if (dr > r-dr) dr = r-dr;
	if ((dr << 1) == r) k++;

	dc = a2-b2;
	if (dc < 0) dc = -dc;
	if (dc > c-dc) dc = c-dc;
	if ((dc << 1) == c) k++;

	m = dc; if (dr < dc) m = dr;
	printf("%d\n", (combi(dr+dc, m) << k) % M);

	return 0;
}