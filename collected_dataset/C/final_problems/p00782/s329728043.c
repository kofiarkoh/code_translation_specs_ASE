// AOJ 1202: Mobile Phone Coverage
// 2017.10.31 bal4u@uu
// IMOS method

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BASE 2000
int map[6004][6004];
int xmin, xmax, ymin, ymax;
char buf[32], *p;

int getint()
{
	int n = 0;
	while (*p >= '0') n = 10*n + (*p++ & 0xf);
	n *= 10;
	if (*p == '.') n += *++p & 0xf, p++;
	return n;
}

int main()
{
	int n, x, y, r, ans;
	int cno = 0;

	while (fgets(buf, sizeof(buf), stdin) && *buf != '0') {
		n = atoi(buf);
		memset(map, 0, sizeof(map));
		xmin = ymin = 6000, xmax = ymax = 0;
		while (n--) {
			fgets(p=buf, sizeof(buf), stdin);
			x = getint()+BASE, p++, y = getint()+BASE, p++, r = getint();
			if (x-r < xmin) xmin = x-r;
			if (x+r > xmax) xmax = x+r;
			if (y-r < ymin) ymin = y-r;
			if (y+r > ymax) ymax = y+r;
			map[y-r][x-r]++;
			map[y-r][x+r]--;
			map[y+r][x-r]--;
			map[y+r][x+r]++;
		}
		for (y = ymin; y <= ymax; y++) for (x = xmin; x <= xmax; x++) map[y][x] += map[y][x-1];
		for (y = ymin; y <= ymax; y++) for (x = xmin; x <= xmax; x++) map[y][x] += map[y-1][x];
		for (ans = 0, y = ymin; y <= ymax; y++) for (x = xmin; x <= xmax; x++)
			if (map[y][x] > 0) ans++;
		printf("%d %d.%02d\n", ++cno, ans/100, ans%100);
	}
	return 0;
}