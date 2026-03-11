// AOJ DSL_5_B The Maximum Number of Overlaps
// 2018.5.5 bal4u

#include <stdio.h>

#if 1
#define gc() getchar_unlocked()
#else
#define gc() getchar()
#endif
int in()
{
	int n = 0, c = gc();
	do n = 10*n + (c & 0xf), c = gc(); while (c >= '0');
	return n;
}

int s[1002][1002];

/* imos法で解く
   いもす法では，矩形の左上 に +1 を，右上 に −1 を，
   左下 に −1 を，右下 に +1 を加算して、累積和を取る．
   記録には O(N) が，累積和の計算には O(WH) がかかり，
   全体の計算量は O(N+WH)．
*/
int main()
{
	int N, i, ans;
	int x, y, xmin, ymin, xmax, ymax;

	xmin = ymin = 1000,	xmax = ymax = 0;

	N = in();
	for (i = 0; i < N; i++) {

		// 左上(x1,y1)、右下(x2, y2)。原点は左上
		int x1 = in(), y1 = in(), x2 = in(), y2 = in();
		s[y1][x1]++, s[y2][x1]--;
		s[y1][x2]--, s[y2][x2]++;

		if (x1 < xmin) xmin = x1;
		if (x2 > xmax) xmax = x2;
		if (y1 < ymin) ymin = y1;
		if (y2 > ymax) ymax = y2;
	}

	// x軸方向の累積和
	for (y = ymin; y <= ymax; y++) for (x = xmin+1; x <= xmax; x++)
		s[y][x] += s[y][x-1];

	// y軸方向の累積和
	for (y = ymin+1; y <= ymax; y++) for (x = xmin; x <= xmax; x++)
		s[y][x] += s[y-1][x];
	
	ans = 0;
	for (y = ymin; y <= ymax; y++) for (x = xmin; x <= xmax; x++)
		if (s[y][x] > ans) ans = s[y][x];

	printf("%d\n", ans);
	return 0;
}

