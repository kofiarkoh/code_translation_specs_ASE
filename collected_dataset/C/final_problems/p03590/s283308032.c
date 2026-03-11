#include <stdio.h>

#define max(a, b) ((a) > (b) ? a : b)

int a[100000], b[100000];

int main(void) {
	int i, j;
	int n, k;
	int num_bit, l;
	long long ans, cand;

	scanf("%d %d", &n, &k);

	num_bit = 0;
	for (i = 1; i <= k; i <<= 1)
		num_bit += !!(k & i);

	for (i = 0; i < n; i++)
		scanf("%d %d", a + i, b + i);

	ans = 0;
	l = 0;
	for (j = 0; j < num_bit; j++) {
		int msb, mask;

		for (msb = 1; msb <= k; msb <<= 1) ;
		msb >>= 1;

		mask = l + msb - 1;

		cand = 0;
		for (i = 0; i < n; i++)
			if (a[i] <= mask) {
				int x, ok;

				ok = 1;
				for (x = 1; x <= mask; x <<= 1)
					if (!(mask & x))
						ok &= !(a[i] & x);
				if (ok)
					cand += b[i];
			}

		l += msb;
		k -= msb;

		ans = max(ans, cand);
	}

	cand = 0;
	for (i = 0; i < n; i++)
		if (a[i] <= l) {
			int x, ok;

			ok = 1;
			for (x = 1; x <= l; x <<= 1)
				if (!(l & x))
					ok &= !(a[i] & x);
			if (ok)
				cand += b[i];
		}

	ans = max(ans, cand);

	printf("%lld\n", ans);

	return 0;
}
