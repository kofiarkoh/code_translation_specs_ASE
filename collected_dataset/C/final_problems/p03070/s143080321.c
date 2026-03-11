#if 0
cat <<EOF >mistaken-paste
#endif
// thx Ebi-chan!

#pragma GCC diagnostic ignored "-Wincompatible-pointer-types"
#define _USE_MATH_DEFINES

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>

#define FOD  998244353
typedef uint64_t ull;
typedef  int64_t sll;

#define N_MAX 1048576

ull n;

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

sll a[N_MAX + 5];

ull dp[100000], dq[100000];

ull solve () {
	sll i, j, ki, li;
	sll result = 0;
	double dresult = 0;
	sll maybe = 0;
	sll sum = 0;
	sll item;
	sll *dpcell;

	for (i = 0; i < n; i++) {
		sum += a[i];
	}

	dp[0] = dq[0] = 1;
	for (i = 0; i < n; i++) {
		for (j = sum; j >= 0; j--) {
			dp[j] *= 2;
			if (j >= a[i]) {
				dp[j] += dp[j - a[i]];
				dq[j] += dq[j - a[i]];
			}

			dp[j] %= FOD;
			dq[j] %= FOD;
		}
	}

	for (j = sum; j * 2 >= sum; j--) {
		result += dp[j];
		result %= FOD;
	}
	if (sum % 2 == 0) {
		result += (FOD - dq[sum / 2]) * 2;
	}
	result *= 3;
	result %= FOD;
	if (sum % 2 == 0) {
		result = (result + dq[sum / 2] * 3) % FOD;
	}

	result = (FOD + bitpow(3, n, FOD) - result) % FOD;

	printf("%lld\n", result);

	return 0;
}

int32_t main (int argc, char *argv[]) {
	int32_t i, j;

	scanf("%llu", &n);
	for (i = 0; i < n; i++) {
		scanf("%lld", &a[i]);
	}

	solve();

	return 0;
}
