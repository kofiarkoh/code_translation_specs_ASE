// AOJ 1579 Array Update 2
// 2018.3.20 bal4u

#include <stdio.h>
#include <stdlib.h>

// 高速数値入出力
#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()  // 非負整数の入力
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

typedef struct { int x, y, z; int pos; } Q;
Q q[200005]; int M;
int N; int a, d;
int K;

int main()
{
	int i, pos, ans;

	// 入力処理
	N = in(), a = in(), d = in(), M = in();
	for (i = 0; i < M; i++) q[i].x = in(), q[i].y = in(), q[i].z = in();
	K = in();

	// 最初の数列に当たるK番目の位置を命令文の逆順で計算する
	pos = K;
	for (i = M - 1; i >= 0; i--) {
		if (q[i].x == 0) {
			if (q[i].y <= pos && pos <= q[i].z) pos = q[i].y + q[i].z - pos;
		}
		q[i].pos = pos;
	}

	// 命令文の順で値を確定していく
	ans = a + (pos - 1)*d;
//	printf("a=%d, d=%d, pos=%d, ans=%d\n", a, d, pos, ans);
	for (i = 0; i < M; i++) {
		pos = q[i].pos;
		if (q[i].y <= pos && pos <= q[i].z) {
			if (q[i].x == 1) ans++;
			else if (q[i].x == 2) ans >>= 1;
		}
//		printf("[%d] pos=%d, ans=%d\n", i, pos, ans);
	}
	printf("%d\n", ans);
	return 0;
}
