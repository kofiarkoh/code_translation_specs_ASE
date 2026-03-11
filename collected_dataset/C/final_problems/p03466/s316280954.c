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
ull q;
ull ua, ub, uc, ud, ue, uf;

ull umin (ull x, ull y) {
	return (x < y) ? x : y;
}



ull solve () {
	sll i, j, ki, li;
	// ull result = 0;
	sll result = 0;
	double dresult = 0;
	// ull maybe = 0;
	sll maybe = 0;
	// ull sum = 0;
	sll sum = 0;
	sll item;
	ull *dpcell;

	while (q--) {
		scanf("%llu%llu%llu%llu", &ua, &ub, &uc, &ud);

		if (ua >= ub) {
			ull l = (ua - 1) / (ub + 1) + 1;
			ull k = umin(ua / l, ub);
			ull arem = ua - k * l;
			ull brem = ub - k;
			for (i = uc; i <= ud; i++) {
				bool isa;
				if (i <= (l + 1) * k) {
					if (i % (l + 1)) {
						isa = true;
					} else {
						isa = false;
					}
				} else {
					if (i > ua + ub - brem) {
						isa = false;
					} else {
						isa = true;
					}
				}
				putchar(isa ? 'A' : 'B');
			}
			puts("");
		} else {
			ull l = (ub - 1) / (ua + 1) + 1;
			if (l == 1) {
				for (i = uc; i <= ud; i++) {
					putchar((i % 2) ? 'B' : 'A');
				}
				puts("");
				continue;
			}
			ull befk;
			ull xa;
			ull xb;
			if (ua * l <= ub) {
				befk = 0;
			} else {
				befk = (ua * l - ub - 1) / (l * l - 1);
			}
			ull arem = ua - l * befk;
			ull brem = ub - befk;
			xa = arem - (brem - 1) / l;
			arem -= xa;
			xb = brem - l * arem;

			for (i = uc; i <= ud; i++) {
				bool isa;
				if (i <= befk * (l + 1) + xa) {
					if (i % (l + 1)) {
						isa = true;
					} else {
						isa = false;
					}
				} else {
					if ((ua + ub - i + 1) % (l + 1)) {
						isa = false;
					} else {
						isa = true;
					}
				}
				putchar(isa ? 'A' : 'B');
			}
			puts("");
		}

	}

	return 0;
}

int32_t main (void) {
	scanf("%llu", &q);
	solve();

	return 0;
}
