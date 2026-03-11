#pragma GCC diagnostic ignored "-Wincompatible-pointer-types"
#define _USE_MATH_DEFINES

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>

typedef uint64_t ull;
typedef  int64_t sll;

ull n, q;
char s[1050000];

int nextdp[1050000][27];
int doub[20][1050000];

ull solve () {
	sll i, j, ki, li;
	sll result = 0;
	double dresult = 0;
	sll maybe = 0;
	sll sum = 0;
	sll item;
	sll *dpcell;

	n = strlen(s);

	for (j = 0; j < 27; j++) {
		nextdp[n + 1][j] = n + 1;
		nextdp[n][j] = n + 1;
	}
	nextdp[n][26] = n;

	for (i = n - 1; i >= 0; i--) {
		sll ch = s[i] - 'a';

		nextdp[i][ch] = i + 1;
		for (j = ch + 1; j < 27; j++) {
			nextdp[i][j] = nextdp[nextdp[i][j - 1]][j - 1];
		}
		if (ch) {
			for (j = 0; j < ch; j++) {
				nextdp[i][j] = nextdp[nextdp[i][26]][j];
			}
		}
	}
	for (j = 0; j <= n + 1; j++) {
		doub[0][j] = nextdp[j][26];
	}
	for (i = 1; i < 20; i++) {
		for (j = 0; j <= n + 1; j++) {
			doub[i][j] = doub[i - 1][doub[i - 1][j]];
		}
	}

	while (q--) {
		int l, r;
		scanf("%d%d", &l, &r);
		l--;

		for (i = 19; i >= 0; i--) {
			if (doub[i][l] <= r) {
				l = doub[i][l];
			}
		}
		if (l == r) {
			puts("Yes");
		} else {
			puts("No");
		}
	}
	return 0;
}

int32_t main (int argc, char *argv[]) {
	scanf("%s", s);
	scanf("%llu", &q);

	solve();

	return 0;
}
