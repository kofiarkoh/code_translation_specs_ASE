/*
cat <<EOF >mistaken-paste
*/

#pragma GCC diagnostic ignored "-Wincompatible-pointer-types"

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
typedef uint64_t ull;
typedef  int64_t sll;

#define N_MAX 1000
#define M_MAX 200000

typedef struct {
	int32_t a;
	int32_t b;
} hw;

typedef struct {
	sll a;
	sll b;
} hwll;

typedef struct {
	hwll a;
	hwll b;
} linell;


typedef struct {
	ull s;
	ull t;
	int32_t c;
} struct_a;

typedef struct {
	int32_t from;
	int32_t to;
	sll cost;
} struct_b;

ull n, m;
ull h, w;
ull k;
ull q;
ull vua, vub, vuc, vud, vue, vuf;
sll vsa, vsb, vsc, vsd, vse, vsf;
long double vra, vrb, vrc;
double vda, vdb, vdc;
size_t slen;
size_t tlen;
char ch, dh;

ull umin (ull x, ull y) {
	return (x < y) ? x : y;
}

ull umax (ull x, ull y) {
	return (x > y) ? x : y;
}

sll smin (sll x, sll y) {
	return (x < y) ? x : y;
}

sll smax (sll x, sll y) {
	return (x > y) ? x : y;
}

ull bitpow (ull a, ull x, ull modulo) {
	ull result = 1;
	while (x) {
		if (x & 1) {
			result *= a;
			result %= modulo;
		}
		x /= 2;
		a = (a * a) % modulo;
	}
	return result;
}

ull divide (ull a, ull b, ull modulo) {
	return (a * bitpow(b, modulo - 2, modulo)) % modulo;
}

ull dp[N_MAX + 1];
ull amazing[N_MAX + 100];
ull npkdp[N_MAX + 1][N_MAX + 1];
ull division[N_MAX + 1][N_MAX + 1];

ull npk (ull n, ull k) {
	return divide(amazing[n], amazing[n - k], MOD);
}

ull grouping (sll whole, sll members, sll parts) {
	// whole_P_(mem*par)/(members!)^parts / parts!
	return divide(npk(whole, members * parts), division[members][parts], MOD);
}

ull solve () {
	sll i, j, ki;
	// ull result = 0;
	sll result = 0;
	// double result = 0;
	ull maybe = 0;
	// sll maybe = 0;
	ull sum = 0;
	// sll sum = 0;
	ull item;
	ull *dpcell;


	amazing[0] = 1;
	for (i = 1; i <= n; i++) {
		amazing[i] = (amazing[i - 1] * i) % MOD;
	}
	for (i = vua; i <= vub; i++) {
		for (j = vuc; j <= vud; j++) {
			division[i][j] = (bitpow(amazing[i], j, MOD) * amazing[j]) % MOD;
		}
	}
	// puts("OK");

	sll latest = n;
	dp[n] = 1;

	for (i = vua; i <= vub; i++) {
		latest = smax(latest - i * vud, 0);
		for (j = latest; j <= n - i * vuc; j++) {

			for (ki = vuc; ki <= vud; ki++) {
				sll xi = j + i * ki;
				if (xi > n) break;
				if (!dp[xi]) continue;

				dp[j] += dp[xi] * grouping(xi, i, ki);
				dp[j] %= MOD;
			}
			// printf("[%lld]; %llu\n", j, dp[j]);
		}
		// if (i <= 50 || i % 50 == 0) {
		// 	printf("%lld... [0]:%llu\n", i, dp[0]);
		// 	fflush(stdout);
		// }
	}
	result = dp[0];

	printf("%llu\n", result);

	return 0;
}

int32_t main (void) {
	int32_t i, j;
	int32_t x, y;

	scanf("%llu", &n, &m);
	scanf("%llu%llu%llu%llu", &vua, &vub, &vuc, &vud, &vue, &vuf);

	solve();

	return 0;
}
