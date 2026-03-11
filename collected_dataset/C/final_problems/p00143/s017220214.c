// Aizu Vol-2 0143   Altair and Vega
// 2017.8.8

#include <stdio.h>

int atSameSide(int x1, int y1, int x2, int y2, int xa, int ya, int xb, int yb)
{
	int sa, sb;

	sa = (x2 - x1) * (ya - y1) + (y2 - y1) * (x1 - xa);
	sb = (x2 - x1) * (yb - y1) + (y2 - y1) * (x1 - xb);
//	if (sb == 0) return 0;  		 // xb,yb in line
	return (sa * sb >= 0);           // xa,ya xb,yb at same side
}

int main()
{
	int n;
	int x1, y1, x2, y2, x3, y3, xk, yk, xs, ys;
	int kin, sin;

	scanf("%d", &n);
	while (n-- > 0) {
		scanf("%d%d%d%d%d%d%d%d%d%d", &x1, &y1, &x2, &y2, &x3, &y3, &xk, &yk, &xs, &ys);
		kin = atSameSide(x1, y1, x2, y2, x3, y3, xk, yk) &&
			  atSameSide(x2, y2, x3, y3, x1, y1, xk, yk) &&
			  atSameSide(x3, y3, x1, y1, x2, y2, xk, yk);
		sin = atSameSide(x1, y1, x2, y2, x3, y3, xs, ys) &&
		  	  atSameSide(x2, y2, x3, y3, x1, y1, xs, ys) &&
			  atSameSide(x3, y3, x1, y1, x2, y2, xs, ys);
		puts(kin ^ sin ? "OK" : "NG");
	}
	return 0;
}