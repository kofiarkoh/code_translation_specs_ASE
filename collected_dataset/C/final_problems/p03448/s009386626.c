#include <stdio.h>

int main(void) {
	int A, B, C, X;
	int count = 0;
	int i, j;
	if (scanf("%d", &A) != 1) return 1;
	if (scanf("%d", &B) != 1) return 1;
	if (scanf("%d", &C) != 1) return 1;
	if (scanf("%d", &X) != 1) return 1;
	for (i = 0; i <= A; i++) {
		for (j = 0; j <= B; j++) {
			int nokori = X - 500 * i - 100 * j;
			if (nokori >= 0 && nokori % 50 == 0 && nokori / 50 <= C) count++;
		}
	}
	printf("%d\n", count);
	return 0;
}
