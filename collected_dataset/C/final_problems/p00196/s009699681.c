// Aizu Vol-1 0196: Baseball Championship
// 2017.8.3

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

char *gets(char *);
char buf[500], *p;

int getInt(void)
{
	int n = 0;
	while (isspace(*p)) p++;
	return *p++ - '0';
}

char getStr(void)
{
	while (isspace(*p)) p++;
	return *p++;
}

typedef struct { char a; int k, w, l; } T;
T t[15];

int main()
{
	int i, j, k, n;
	T tmp, *tp, *tt;

	while (1) {
		gets(buf);
		if (!(n = atoi(buf))) break;
		for (i = 0, tp = t; i < n; i++, tp++) {
			gets(p = buf);
			tp->a = getStr(), tp->k = i, tp->w = tp->l = 0;
			for (j = 1; j < n; j++) {
				k = getInt();
				if (k == 0) tp->w++;
				else if (k == 1) tp->l++;
			}
		}
		for (i = 0; i < n; i++) {
			tp = t + i;
			for (j = i+1, tt = tp+1; j < n; j++, tt++) {
				if (tt->w > tp->w) tp = tt;
				else if (tt->w == tp->w) {
					if (tt->l < tp->l) tp = tt;
					else if (tt->l == tp->l && tt->k < tp->k) tp = tt;
				}
			}
			if (tp != t + i) {
				memcpy(&tmp, t+i, sizeof(tmp));
				memcpy(t+i, tp, sizeof(tmp));
				memcpy(tp, &tmp, sizeof(tmp));
			}
		}
		for (i = 0; i < n; i++) putchar(t[i].a), putchar('\n');
	}
	return 0;
}