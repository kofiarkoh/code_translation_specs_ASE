#include <stdio.h>
int main(void) {
	int a, b, k;
	scanf("%d%d%d", &a, &b, &k);
	for (int i = 0; i < k; i++) {
		if (i % 2 == 0) {
			if (a % 2 == 1) {
				a--;
			}
			a = a / 2;
			b = b + a;
		}
		else {
			if (b % 2 == 1) {
				b--;
			}
			b = b / 2;
			a = a + b;
		}
	}
	printf("%d %d", a, b);
	getchar();
	getchar();
	return 0;
}