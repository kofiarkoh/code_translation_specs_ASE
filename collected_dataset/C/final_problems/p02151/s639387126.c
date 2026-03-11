// AOJ 3053 Phone Number
// 2018.3.17 bal4u

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define gc() getchar_unlocked()
#define pc(c) putchar_unlocked(c)
#else
#define gc() getchar()
#define pc(c) putchar(c)
#endif
int in()  // 非負整数の入力
{
	int n = 0, c = gc();
	do n = 10 * n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

void outs(char *s) { while (*s) pc(*s++); }  // 文字列の表示

int K = 9;
int PP[] = { 0, 1,2,3,4,5,6,7,8,9 };   // 0はダミ
int *perm = PP + 1;                    // action()ではPP[1]から利用

/* 与えられた並び PP[] の次に大きい並べ替えを求める */
int next_perm()   // 順列の大きさはK
{
	int i, j, k, t;
	void action();

	while (1) {
		/* 上位桁のほうが下位桁よりも小さいところまで移動 */
		i = K; while (PP[i - 1] >= PP[i]) i--;

		/* 次の並べ替えがない */
		if (i == 1) return 0;

		/* PP[i-1]より値の大きい最も下位の桁をPP[j]とする */
		j = K; while (PP[i - 1] >= PP[j]) j--;

		/* PP[i-1]とPP[j]とを交換 */
		t = PP[i - 1], PP[i - 1] = PP[j], PP[j] = t;

		/* PP[i]から最下位までを逆順 */
		k = (K - i) >> 1; for (j = 0; j <= k; j++)
			t = PP[i + j], PP[i + j] = PP[K - j], PP[K - j] = t;

		action();
	}
	return 1;
}

#define INF 100000000
#define ABS(x) ((x)>=0?(x):-(x))
int N;
int ans[9], min;
int diff[10][10];
int cnt['9'+1]['9'+1];
int tbl[9][9] = {
	{0,1,2,1,2,3,2,3,4},
	{1,0,1,2,1,2,3,2,3},
	{2,1,0,3,2,1,4,3,2},
	{1,2,3,0,1,2,1,2,3},
	{2,1,2,1,0,1,2,1,2},
	{3,2,1,2,1,0,3,2,1},
	{2,3,4,1,2,3,0,1,2},
	{3,2,3,2,1,2,1,0,1},
	{4,3,2,3,2,1,2,1,0} };
void action()
{
	int i, j, k;
	
	for (i = 0; i < 9; i++) {
		k = perm[i];
		for (j = 0; j < 9; j++) diff[k][perm[j]] = tbl[i][j];
	}

	k = 0; for (i = 1; i <= 9; i++) for (j = 1; j <= 9; j++)
		k += diff[i][j] * cnt[i+'0'][j+'0'];

	if (k < min) {
		min = k;
		memcpy(ans, perm, sizeof(ans));
	}
}

int main()
{
	int i, j;

	N = in();
	if (N == 1) { outs("123\n456\n789\n"); return 0; }
	i = gc(); while (--N) j = gc(), cnt[i][j]++, i = j;

	min = INF;
	action();
	next_perm();
//	printf("Min=%d\n", min);

	for (i = 0; i < 9; i++) {
		pc(ans[i]+'0');
		if ((i + 1) % 3 == 0) pc('\n');
	}
	return 0;
}

#if 0
// テストデータを作成する
#include <time.h>
int main()
{
	int i, N = 100000;

	printf("%d\n", N);
	srand((unsigned)time(NULL));
	for (i = 0; i < N; i++) {
		printf("%1d", rand() % 9 + 1);
	}
	printf("\n");
	return 0;
}
#endif

