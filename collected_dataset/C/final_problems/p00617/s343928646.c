// Aizu Vol-10 1031: Simple GUI Application
// 2017.8.19 bal4u@uu

#include <stdio.h>
#include <ctype.h>
#include <string.h>

char *gets(char *);
char buf[100], *p;
int getInt(void)
{
	int n = 0;
	while (isspace(*p) || *p == ',') p++;
	while (isdigit(*p)) n = 10 * n + (*p++ - '0');
	return n;
}

char tag[1005];
int x, y;
char ans[100]; int no;

char *parse(char *tp, int *done, int *k)
{
	int x1, y1, x2, y2;
	int ok = 0;
	int sum = 0, s;
	char name[100], *np;

	*k = 0;
//	while (isspace(*tp)) tp++;
	if (!*tp) return tp;
	tp++;	// '<'
	np = name; while (*tp != '>') *np++ = *tp++; *np = 0;
	tp++;	// '>'
	p = tp, x1 = getInt(), y1 = getInt(), x2 = getInt(), y2 = getInt(),	tp = p;
	if (x1 <= x && x <= x2 && y1 <= y && y <= y2) ok = 1;
	while (*tp) {
//		while (isspace(*tp)) tp++;
		if (*(tp + 1) == '/') break;
		tp = parse(tp, done, &s), sum += s+1;
	}
	while (*tp++ != '>') ;
	if (!*done && ok) strcpy(ans, name), no = sum, *done = 1;
	*k = sum;
	return tp;
}

int main()
{
	int n, i, d, k;

	while (1) {
		gets(p = buf); if (!(n = getInt())) break;
		gets(tag);
		for (i = 0; i < n; i++) {
			gets(p = buf), x = getInt(), y = getInt();
			no = -1, d = 0;
			parse(tag, &d, &k);
			if (no < 0) puts("OUT OF MAIN PANEL 1");
			else printf("%s %d\n", ans, no);
		}
	}
	return 0;
}