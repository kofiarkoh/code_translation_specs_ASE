// AOJ DPL_5_E Balls and Boxes 5
// n 個の区別できないボールを k 個の区別できる箱に入れる
// とき、可能な入れ方の総数を求めてください。
// ただし、
//    どのボールも、必ずいずれかの箱に入れる。
//    各箱に入れることのできるボールの数は高々1つまで。
// 答えは組み合わせ combi(k, n)
// 2019.3.11 bal4u

#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007
int tbl[1001][1001];

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
	if (n > k) puts("0");
	else printf("%d\n", combi(k, n));
	return 0;
}
