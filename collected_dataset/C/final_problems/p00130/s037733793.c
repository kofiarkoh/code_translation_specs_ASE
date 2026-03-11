// Aizu Vol-1 0130: Train
// 2017.8.3

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

char *gets(char *);
char buf[1100], *p;
char ans[100], *a, *t;

int main()
{
	int i, n;

	gets(buf);
	n = atoi(buf);
	for (i = 0; i < n; i++) {
		gets(p = buf);
		memset(ans, 0, sizeof(ans));
		t = a = ans + 50;
		while (*p) {
			if (isspace(*p)) p++;
			if (!*p) break;
			if (isalpha(*p)) {
				if (!*a) *a = *p;
				else if (*a != *p) while (1);
			}
			else if (*p == '<') {
				if (*++p != '-') while (1);
				a--;
				if (a < t) t = a;
			}
			else if (*p == '-') {
				if (*++p != '>') while (1);
				a++;
			}
			p++;
		}
		puts(t);
	}
	return 0;
}