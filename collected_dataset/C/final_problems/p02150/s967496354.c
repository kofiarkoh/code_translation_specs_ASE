#include <stdio.h>
#include <inttypes.h>

#define MOD_BY INT64_C(1000000007)

int64_t add(int64_t a, int64_t b) {
	return (a + b) % MOD_BY;
}

int64_t mul(int64_t a, int64_t b) {
	int64_t r = 0;
	while (b > 0) {
		if (b & 1) r = add(r, a);
		a = add(a, a);
		b >>= 1;
	}
	return r;
}

int main(void) {
	int64_t a, b, x;
	if (scanf("%" PRId64 "%" PRId64 "%" PRId64, &a, &b, &x) != 3) return 1;
	if (x < a) {
		printf("%" PRId64 "\n", x % MOD_BY);
	} else {
		int64_t yoryoku = x - a;
		int64_t koukan = yoryoku / (a - b) + 1;
		printf("%" PRId64 "\n", add(x, mul(b, koukan)));
	}
	return 0;
}

