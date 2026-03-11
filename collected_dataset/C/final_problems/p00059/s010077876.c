// Aizu Vol0 0059: Intersection of Rectangles
// 2017.7.30

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

char *gets(char *);

#define MAX 500
char buf[MAX + 5], *p;
#define MAGIC 5

int ax1, ay1, ax2, ay2;

int GetInt(void)
{
	int minus = 0, len = 0, n = 0;

	while (isspace(*p)) p++;
	if (*p == '-') p++, minus = 1;
	while (isdigit(*p)) n = 10 * n + (*p++ - '0');
	if (*p == '.') p++;
	while (isdigit(*p)) n = 10 * n + (*p++ - '0'), len++;
	while (len < MAGIC) n = 10 * n, len++;
	if (minus) n = -n;
//	printf("%10d\n", n);
	return n;
}

int InOn(int x, int y)
{
	if ((ax1 <= x && x <= ax2) && (ay1 <= y && y <= ay2)) return 1;
	return 0;
}

int main()
{

	int bx1, by1, bx2, by2;
	int t;

	while (gets(p = buf) != NULL) {
		ax1 = GetInt(), ay1 = GetInt();
		ax2 = GetInt(), ay2 = GetInt();
		bx1 = GetInt(), by1 = GetInt();
		bx2 = GetInt(), by2 = GetInt();

		if (ax1 > ax2) t = ax1, ax1 = ax2, ax2 = t;
		if (ay1 > ay2) t = ay1, ay1 = ay2, ay2 = t;
		if (bx1 > bx2) t = bx1, bx1 = bx2, bx2 = t;
		if (by1 > by2) t = by1, by1 = by2, by2 = t;

		if (InOn(bx1, by1)) goto YES;
		if (InOn(bx2, by1)) goto YES;
		if (InOn(bx2, by2)) goto YES;
		if (InOn(bx1, by2)) goto YES;

		puts("NO");
		continue;
YES:	puts("YES");
	}
	return 0;
}