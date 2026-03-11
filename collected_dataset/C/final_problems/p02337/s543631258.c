// AOJ DPL_5_G: Balls and Boxes 7
// n 個の区別できるボールを k 個の区別できない箱に入れる
// とき、可能な入れ方の総数を求めてください。
// ただし、
//    どのボールも、必ずいずれかの箱に入れる。
//    ボールが1つも入っていない箱があってもよい。
// 2019.3.11 bal4u

#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007
int tbl[1001][1001];

int stirling2(int n, int k)
{
	int ans;

	if (k > n || k == 0) return 0;
	if (tbl[n][k]) return tbl[n][k];
	if (k == 1 || k == n) ans = 1;
	else ans = (((long long)k * stirling2(n - 1, k)) % MOD)
		+ stirling2(n - 1, k - 1);
	return tbl[n][k] = ans % MOD;
}

int main()
{
	int n, k, ans;

	scanf("%d%d", &n, &k);
	ans = 0; while (k > 0) {
		ans += stirling2(n, k--);
		if (ans >= MOD) ans -= MOD;
	}
	printf("%d\n", ans);
	return 0;
}
