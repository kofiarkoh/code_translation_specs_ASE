/*
cat <<EOF >mistaken-paste
*/

#pragma GCC diagnostic ignored "-Wincompatible-pointer-types"
#define _USE_MATH_DEFINES

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>

#define BIG 2000000007
#define VERYBIG 200000000000007LL

#define MOD 1000000007
#define FOD  998244353
typedef uint64_t ull;
typedef  int64_t sll;

#define N_MAX 300000
#define M_MAX 200000

ull n, m;


sll a[N_MAX];
sll b[N_MAX];

ull deg[N_MAX];
ull exm[N_MAX];
ull lvs[N_MAX];

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

	for (i = 0; i < n - 1; i++) {
		deg[a[i]]++;
		exm[a[i]] = b[i];

		deg[b[i]]++;
		exm[b[i]] = a[i];
	}
	for (i = 0; i < n; i++) {
		if (deg[i] == 1) {
			if (++lvs[exm[i]] > 1) goto success;
		}
	}

	goto fail;

	success:
	puts("First");
	return 0;

	fail:
	puts("Second");
	return 1;
}

int32_t main (void) {
	int32_t i;
	scanf("%llu", &n, &m);
	for (i = 0; i < n - 1; i++) {
		scanf("%lld", &a[i]);
		scanf("%lld", &b[i]);
		a[i]--;
		b[i]--;
	}

	solve();

	return 0;
}
