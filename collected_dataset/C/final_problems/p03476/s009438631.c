#include <stdio.h>

int prime(int n) {
	int a;

	for (a = 2; a <= n / a; a++)
		if (n % a == 0)
			return 0;
	return 1;
}

int main() {
	static int aa[100000];
	int q, a;

	scanf("%d", &q);
	aa[1] = 0;
	for (a = 3; a < 100000; a += 2) {
		aa[a] = aa[a - 2];
		if (prime(a) && prime((a + 1) / 2))
			aa[a]++;
	}
	while (q--) {
		int l, r;

		scanf("%d%d", &l, &r);
		printf("%d\n", aa[r] - aa[l - 2]);
	}
	return 0;
}
