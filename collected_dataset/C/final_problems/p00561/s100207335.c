// AOJ 0638 Kingdom of JOIOI
// 2018.1.7 bal4u

#include <stdio.h>

#define INF 1000000001

int H, W;
int a[2002][2002];
int min, max;

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int check(int x)
{
	int r, c;
	int f = 0;
	for (r = 0; r < H; r++) {
		for (c = 0; c < W; c++) {
			if (a[r][c] < max - x) if (f < c+1) f = c+1;
		}
		for (c = 0; c < W; c++) {
			if (a[r][c] > min + x) if (c < f) return 0;
		}
	}
	return 1;
}

void flip_row()
{
	int r, c, t, h2;
	h2 = H >> 1;
	for (r = 0; r < h2; r++) for (c = 0; c < W; c++) {
		t = a[r][c], a[r][c] = a[H-1-r][c], a[H-1-r][c] = t;
	}
}

void flip_col()
{
	int r, c, t, w2;
	w2 = W >> 1;
	for (r = 0; r < H; r++) for (c = 0; c < w2; c++) {
		t = a[r][c], a[r][c] = a[r][W-1-c], a[r][W-1-c] = t;
	}
}

int solve()
{
	int lo, hi, mi;
	
	lo = 0, hi = max - min;
	while (hi-lo > 1) {
		mi = (lo + hi) >> 1;
		if (check(mi)) hi = mi; else lo = mi;
	}
	return hi;
}

int main()
{
	int r, c, v, t, ans;

	H = in(), W = in();
	min = INF, max = 0;
	for (r = 0; r < H; r++) for (c = 0; c < W; c++) {
		a[r][c] = v = in();
		if (min > v) min = v;
		if (max < v) max = v;
	}
	ans = solve();
	flip_row();	t = solve(); if (t < ans) ans = t;
	flip_col();	t = solve(); if (t < ans) ans = t;
	flip_row();	t = solve(); if (t < ans) ans = t;
	printf("%d\n", ans);
	return 0;
}

