// AOJ DPL_5_D Balls and Boxes 4
// n 個の区別できないボールを k 個の区別できる箱に入れる
// とき、可能な入れ方の総数を求めてください。
// 答えは重複組み合わせ kHn = combi(k+n-1, n)
// 2019.3.11 bal4u

#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007
int tbl[2001][1001];

int combi(int n, int k)
{
	int ans;

	if (tbl[n][k]) return tbl[n][k];
	if ((k << 1) > n) k = n - k;
	if (k == 0) ans = 1;
	else if (k == 1) ans = n;
	else ans = combi(n - 1, k) + combi(n - 1, k - 1);
	return tbl[n][k] = ans % MOD;
}

int main()
{
	int n, k;

	scanf("%d%d", &n, &k);
	printf("%d\n", combi(n + k - 1, n));
	return 0;
}
