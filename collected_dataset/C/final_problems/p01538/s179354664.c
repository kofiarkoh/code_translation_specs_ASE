#include <stdio.h>

#define max(a, b) (((a) > (b)) ? (a) : (b))

int main(void) {
	int i;
	int q;

	scanf("%d", &q);

	while (q--) {
		int n;
		int cnt;

		scanf("%d", &n);

		cnt = 0;
		while (n > 9) {
			int max = 0;
			for (i = 10; i <= 1000000; i *= 10)
				max = max(max, n / i * (n % i));
			n = max;

			cnt++;
		}

		printf("%d\n", cnt);
	}

	return 0;
}