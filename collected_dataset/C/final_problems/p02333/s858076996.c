// AOJ DPL_5_C Balls and Boxes 3
// n 個の区別できるボールを k 個の区別できる箱に入れる
// とき、可能な入れ方の総数を求めてください。
// ただし、
//    どのボールも、必ずいずれかの箱に入れる。
//    どの箱にも、1つ以上のボールを入れる。
// 答えは　階乗*第2種スターリング数 k! * stirling2(n, k)
// 2019.3.11 bal4u

#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007
int tbl[1001][1001];

int fact(int n)
{
	int i;
	long long ans = 1;
	for (i = 2; i <= n; i++) ans = (ans * i) % MOD;
	return (int)ans;
}

int stirling2(int n, int k)
{
	int ans;

	if (k == 0) return 0;
	if (tbl[n][k]) return tbl[n][k];
	if (k == 1 || k == n) ans = 1;
	else ans = (((long long)k * stirling2(n - 1, k)) % MOD)
		+ stirling2(n - 1, k - 1);
	return tbl[n][k] = ans % MOD;
}

int main()
{
	int n, k;

	scanf("%d%d", &n, &k);
	if (n < k) puts("0");
	else printf("%d\n", (int)(((long long)fact(k)*stirling2(n, k)) % MOD));
	return 0;
}
