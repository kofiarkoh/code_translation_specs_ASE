// AOJ 1248: The Balance
// 2017.10.17 bal4u@uu

#include <stdio.h>
#include <string.h>

#define ABS(a) ((a)>=0?(a):-(a))

int extendedGCD(int a, int b, int *x, int *y)
{
	int d;
	if (b == 0) { *x = 1; *y = 0; return a; }
	d = extendedGCD(b, a % b, y, x);
	*y -= a / b * (*x);
	return d;
}

int main()
{
	int a, b, d, x, y;
	int d0, x0, y0, x1, y1, x2, y2, t, min;

	while (scanf("%d%d%d", &a, &b, &d) && a > 0) {
		d0 = extendedGCD(a, b, &x0, &y0);
		d /= d0, a /= d0, b /= d0;
		// x = d*x0 + b*t;
		// y = d*y0 - a*t;

		t = 1 + (d*(x0+y0) / (a-b));
		x1 = d*x0 + b*t; x = ABS(x1);
		y1 = d*y0 - a*t; y = ABS(y1);
		min = x + y;
		do {
			t--;
			x1 = d*x0 + b*t; x2 = ABS(x1);
			y1 = d*y0 - a*t; y2 = ABS(y1);
			if (x2 + y2 < min) x = x2, y = y2, min = x+y;
		} while (x2 <= x || y2 <= y);
		printf("%d %d\n", x, y);
	}
 	return 0;
}