// AOJ 2110: Radio Base
// 2017.12.29 bal4u@uu

#include <stdio.h>

#define ABS(a) ((a)>=0?(a):-(a))

int x1, y1, x2, y2, x, y;
char buf[80], *p;

int in()
{
	int n = 0;
	if (*p == '-') {
		p++; while (*p >= '0') n = 10*n + (*p++ & 0xf);
		n = -n;
	} else while (*p >= '0') n = 10*n + (*p++ & 0xf);
	p++;
	return n;
}

int calc(int x, int y)
{
	int f = 0;
	if (x == x1 || y == y1 || ABS(x-x1) == ABS(y-y1)) f |= 1;
	if (x == x2 || y == y2 || ABS(x-x2) == ABS(y-y2)) f |= 2;
	return f;
}

int _chk(int x3, int y3)
{
	return x == x3 || y == y3 || ABS(x-x3) == ABS(y-y3);
}

int check()
{
	if (_chk(x1, y2) || _chk(x2, y1)) return 1;
	if (x2 > x1) {
		if (_chk(x2, y1+x2-x1) || _chk(x2, y1-(x2-x1))) return 1;
		if (_chk(x1, y2+x2-x1) || _chk(x1, y2-(x2-x1))) return 1; 
	} else {
		if (_chk(x2, y1+x1-x2) || _chk(x2, y1-(x1-x2))) return 1;
		if (_chk(x1, y2+x1-x2) || _chk(x2, y2-(x1-x2))) return 1;
	}
	if (y2 > y1) {
		if (_chk(x1+y2-y1, y2) || _chk(x1-(y2-y1), y2)) return 1;
		if (_chk(x2+y2-y1, y1) || _chk(x2-(y2-y1), y1)) return 1; 
	} else {
		if (_chk(x1+y1-y2, y2) || _chk(x1-(y1-y2), y2)) return 1;
		if (_chk(x2+y1-y2, y1) || _chk(x2-(y1-y2), y1)) return 1;
	}
	if (x1 == x2 || y1 == y2 || ABS(x1-x2) == ABS(y1-y2)) return 1;

	return 0;
}

int main()
{
	int n, x3, y3, f;

	fgets(p=buf, 10, stdin), n = in();
	while (n--) {
		fgets(p=buf, 80, stdin);
		x1 = in(), y1 = in(), x2 = in(), y2 = in(), x = in(), y = in();
		if (x == x1 && y == y1 || x == x2 && y == y2) { puts("0"); continue; }
		f = calc(x, y);
		if (f == 3) { puts("1"); continue; }
		if (f > 0 ) { puts("2"); continue; }

		f = check();
		if (!f) {
			if (y > y2) {
				y3 = y, x3 = x2 + y-y2;
				f |= (ABS(x3-x1) == ABS(y3-y1));
			} else {
				y3 = y, x3 = x2 + y2-y;
				f |= (ABS(x3-x1) == ABS(y3-y1));
			}
			if (x > x2) {
				x3 = x, y3 = y2 + x-x2;
				f |= (ABS(x3-x1) == ABS(y3-y1));
			} else {
				x3 = x, y3 = y2 + x2-x;
				f |= (ABS(x3-x1) == ABS(y3-y1));
			}
		}
		if (f) puts("3");
		else puts("4");
	}
	return 0;
}