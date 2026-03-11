// AOJ 0348: Sevens
// 2017.10.28 bal4u@uu

#include <stdio.h>
#include <string.h>

char c[14];

int check()
{
	int min, max, an, bn;

	if (c[1] && c[13]) return 0;
	if (!c[1] && !!c[13]) return 1;
	min = 6, max = 8, an = bn = 6;
	if (c[1]) {
		while (1) {
			if      (min >=  1 &&  c[min]) min--, an--;
			else if (max <= 13 &&  c[max]) max++, an--;
			if (an == 0) return 1;
			if      (max <= 13 && !c[max]) max++, bn--;
			else if (min >=  1 && !c[min]) min--, bn--;
			if (bn == 0) return 0;
		}
	} else {
		while (1) {
			if      (max <= 13 &&  c[max]) max++, an--;
			else if (min >=  1 &&  c[min]) min--, an--;
			if (an == 0) return 1;
			if      (min >=  1 && !c[min]) min--, bn--;
			else if (max <= 13 && !c[max]) max++, bn--;
			if (bn == 0) return 0;
		}
	}
	return 0;
}

int main()
{
	int n, i, a;

	scanf("%d", &n);
	while (n-- > 0) {
		memset(c, 0, sizeof(c));
		for (i = 0; i < 6; i++)	scanf("%d", &a), c[a] = 1;
		puts(check() ? "yes" : "no"); 
	}
	return 0;
}