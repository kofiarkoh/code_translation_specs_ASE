// AOJ 1224: Starship Hakodate-maru
// 2017.10.15

#include <stdio.h>
#include <string.h>

int cubic[55];
int tetrahedral[97];
char f[153000];  // 152096 = 96*97*98/6

int main()
{
	int n, i, j;

	for (i = 1; i < 55; i++) cubic[i] = i*i*i, f[cubic[i]] = 1;
	for (i = 1; i < 97; i++) tetrahedral[i] = i*(i+1)*(i+2)/6, f[tetrahedral[i]] = 1;
	for (i = 1; i < 55; i++) for (j = i; j < 97; j++) {
		n = cubic[i]+tetrahedral[j];
		if (n < 153000) f[n] = 1;
	}

	while (scanf("%d", &n) && n > 0) {
		while (1) {
			if (f[n]) break;
			n--;
		}
		printf("%d\n", n);
	}
	return 0;
}