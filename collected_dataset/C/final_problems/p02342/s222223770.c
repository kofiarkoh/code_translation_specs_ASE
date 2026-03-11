// AOJ DPL_5_L Balls and Boxes 12
// n 個の区別できないボールを k 個の区別できない箱に入れる
// とき、可能な入れ方の総数を求めてください。
// ただし、
//    どのボールも、必ずいずれかの箱に入れる。
//    どの箱にも、1つ以上のボールを入れる。
// 答えは制限付き分割数 divnum(n-k, k)
// 2019.3.11 bal4u

#include <stdio.h>
#include <stdlib.h>

#define MOD 1000000007
int tbl[1001][1001];

int divnum(int n, int k)
{
	int ans;

	if (tbl[n][k]) return tbl[n][k];
	if (n < 0) return 0;
	if (n == 0 || n == 1 || k == 1) return 1;
	else {
		ans = divnum(n, k - 1);
		if (n >= k) ans += divnum(n - k, k);
	}
	return tbl[n][k] = ans % MOD;
}

int main()
{
	int n, k;

	scanf("%d%d", &n, &k);
	if (n < k) puts("0");
	else printf("%d\n", divnum(n-k, k));
	return 0;
}
