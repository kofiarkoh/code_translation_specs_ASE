// AOJ 0348: Sevens
// 2017.10.28 bal4u@uu

#include <stdio.h>
#include <string.h>

char a[15], b[15];

int check()
{
	int min, max, an, bn;

	if (a[1] && a[13]) return 0;
	if (b[1] && b[13]) return 1;
	min = 6, max = 8, an = bn = 6;
	if (a[1]) {
		while (1) {
			if      (a[min]) min--, an--;
			else if (a[max]) max++, an--;
			if (an == 0) return 1;
			if      (b[max]) max++, bn--;
			else if (b[min]) min--, bn--;
			if (bn == 0) return 0;
		}
	} else {
		while (1) {
			if      (a[max]) max++, an--;
			else if (a[min]) min--, an--;
			if (an == 0) return 1;
			if      (b[min]) min--, bn--;
			else if (b[max]) max++, bn--;
			if (bn == 0) return 0;
		}
	}
	return 0;
}

int main()
{
	int n, i, v;

	scanf("%d", &n);
	while (n-- > 0) {
		memset(a, 0, sizeof(a)), memset(b, 0, sizeof(b));
		for (i = 0; i < 6; i++)	scanf("%d", &v), a[v] = 1;
		for (i = 1; i <= 13; i++) if (!a[i]) b[i] = 1;
		puts(check() ? "yes" : "no"); 
	}
	return 0;
}