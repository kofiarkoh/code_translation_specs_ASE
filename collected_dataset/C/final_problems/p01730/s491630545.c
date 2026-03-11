// AOJ 2618: Trip to Kyoto
// 2018.1.25 bal4u@uu

#include <stdio.h>

#define MIN(a,b)  ((a)<=(b)?(a):(b))

int x1[10002], y1[10002], x2[10002], y2[10002];
char *msg[2] = { "%d\n", "%d.5\n" };

//#define getchar_unlocked()  getchar()
int in()
{
	int n = 0;
	int c = getchar_unlocked();
	if (c == '-') {
		c = getchar_unlocked();
		do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
		while (c >= '0');
		return -n;
	}
	do n = (n<<3)+(n<<1) + (c & 0xf), c = getchar_unlocked();
	while (c >= '0');
	return n;
}

int calc(int x1, int y1, int x2, int y2)
{
	int dx, dy;

	x1 += 200000000, x2 += 200000000;
	y1 += 200000000, y2 += 200000000;
	dx = x1-x2; if (dx < 0) dx = -dx;
	dy = y1-y2; if (dy < 0) dy = -dy;
	if (!(x1%20 | x2%20)) {
		if (x1 == x2) return dy;
		else if (y1/20 == y2/20) return dx + MIN(y1%20+y2%20, 40-y1%20-y2%20);
	} else if (!(y1%20 | y2%20)) {
		if (y1 == y2) return dx;
		else if (x1/20 == x2/20) return dy + MIN(x1%20+x2%20, 40-x1%20-x2%20);
	}
	return dx+dy;
}

int main()
{
	int n, x, y, i, j, k, s, t, ans;
	int xmin, xmax, ymin, ymax;
	int x0, y0;

	n = in();
	x = in() << 1, y = in() << 1;
	x1[0] = x, y1[0] = y, xmin = xmax = x2[0] = x+y, ymin = ymax = y2[0] = x-y;
	for (i = 1; i < n; i++) {
		x = in() << 1, y = in() << 1;
		x1[i] = x,   y1[i] = y;
		x2[i] = x+y, y2[i] = x-y;
		if      (x2[i] < xmin) xmin = x2[i];
		else if (x2[i] > xmax) xmax = x2[i];
		if      (y2[i] < ymin) ymin = y2[i];
		else if (y2[i] > ymax) ymax = y2[i];
	}
 
	x = (xmin + xmax) >> 1, y = (ymin + ymax) >> 1;
	x0 = (x + y) >> 1, y0 = (x - y) >> 1;

#define OFF 10
	ans = 0x7fffffff;
	for (i = x0-OFF; i <= x0+OFF; i++)for (j = y0-OFF; j <= y0+OFF; j++) {
		if (i%20 && j%20) continue;
		t = 0; for (k = 0; k < n; k++) if (t < (s = calc(i, j, x1[k], y1[k]))) t = s;
		if (ans > t) ans = t;
	}
	printf(msg[ans & 1], ans >> 1);
	return 0;
}
