#include <stdio.h>

#ifndef DEBUG_PRINT
#define DEBUG_PRINT 0
#endif

typedef struct {
	int l, r;
} miyage_t;

int N, M;
miyage_t miyage[1000000];

int kaeru[10000000];

int main(void) {
	int i;
	if (scanf("%d%d", &N, &M) != 2) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d%d", &miyage[i].l, &miyage[i].r) != 2) return 1;
	}

	for (i = 0; i < N; i++) {
		int d = 1;
#if DEBUG_PRINT
		printf("debug meisan = [%d, %d]\n", miyage[i].l, miyage[i].r);
#endif
		while (miyage[i].r >= d) {
			int a, nextd;
			/* 次に変化するdを求める */
			if ((miyage[i].l - 1) / d < miyage[i].r / d) {
				/* 買える */
				a = miyage[i].r / d;
				nextd = (miyage[i].r + a) / a - 1;
				/* いもす */
				kaeru[d]++;
				kaeru[nextd + 1]--;
#if DEBUG_PRINT
				printf("can buy [%d, %d]\n", d, nextd);
#endif
			} else {
				/* 買えない */
				a = (miyage[i].l - 1) / d;
				nextd = ((miyage[i].l - 1) + a) / a - 1;
#if DEBUG_PRINT
				printf("cannot buy [%d, %d]\n", d, nextd);
#endif
			}
			/* 次の区間を処理する */
			if (nextd < d) { puts("unko!");return 1;}
			d = nextd + 1;
		}
	}

	/* いもす */
	for (i = 1; i <= M; i++) {
		kaeru[i] += kaeru[i - 1];
		printf("%d\n", kaeru[i]);
	}

	return 0;
}

/*
名産品を買える幅が1 (l == r)の場合は
l % d == 0 のとき、かつそのときに限り買える

一般に、名産品は r % d <= r - l のとき、かつその時に限り買える

-----

x / d < a のとき d > x / a

*** TLE ***

1個の電車で1個の名産品を買えるか判定するには
(l-1)/d < r/d のとき、かつそのときに限り買える
この式はl～rの中にdの倍数があるため、l-1のときよりrの時の方がブロック数が増えていることを示す。

各電車について1個の名産品を買えるか判定するには？
二分探索的な感じで
電車d=1なら絶対買える
dを増やすと、(l-1)/dもr/dも減る (少なくとも、増えない)
買える -> 次にr/dが減るまでは買える、減ったらまた買えるか判定
買えない -> 次に(l-1)/dが減るまでは買えない、減ったらまた買えるか判定

いもす
*/
