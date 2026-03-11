// Aizu 2188: Unit Converter
// 2017.9.20 bal4u@uu

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

char *pre[20] = { 
	"yotta", "zetta", "exa", "peta", "tera",
	"giga", "mega", "kilo", "hecto", "deca",
	"deci", "centi", "milli", "micro", "nano",
	"pico", "femto", "ato", "zepto", "yocto" };
int idx[20] = {
	24, 21, 18, 15, 12,  9,   6,   3,   2,	 1,
	-1, -2, -3, -6, -9,-12, -15, -18, -21, -24 };
 
char buf[1105], ans[1105];

int main()
{
	int t, i, a, b, f;
	char *p, *q, *w;

	fgets(buf, 10, stdin); t = atoi(buf);
	while (t-- > 0) {
		fgets(buf, 1100, stdin); p = buf, q = ans;
		while (isspace(*p)) p++;
		while (*p == '0') p++;
		a = b = 0, f = 0;
		if (*p > '0') *q++ = *p++, f = 1;
		while (isdigit(*p)) {
			if (f == 1) *q++ = '.', f = 2;
			*q++ = *p++, a++;
		}
		if (*p == '.') { if (f == 1) *q++ = '.', f = 2; p++; }
		if (!f) a = -1;
		while (*p == '0') {
			if (f) *q++ = *p;
			else a--;
			p++;
		}
		if (isdigit(*p)) { if (!f) *q++ = *p++, f = 1; }
		while (isdigit(*p)) {
			if (f == 1) *q++ = '.', f = 2;
			*q++ = *p++;
		}
		while (isspace(*p)) p++;
		w = p; while (isalpha(*p)) p++;
		*p = 0;
		for (i = 0; i < 20; i++) if (!strcmp(w, pre[i])) { b = idx[i]; break; }
		if (i < 20) { p++; while (isspace(*p)) p++; w = p; }
		sprintf(q, " * 10^%d ", a + b);
		q += strlen(q);
		while (isalpha(*w)) *q++ = *w++;
		*q = 0;
		puts(ans);
	}
	return 0;
}