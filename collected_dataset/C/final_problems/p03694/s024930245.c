#include <stdio.h>

int main(void) {
	int N;
	int a[128];
	int max, min;
	int i;
	if (scanf("%d", &N) != 1) return 1;
	for (i = 0; i < N; i++) {
		if (scanf("%d", &a[i]) != 1) return 1;
	}
	max = min = a[0];
	for (i = 1; i < N; i++) {
		if (a[i] < min) min = a[i];
		if (a[i] > max) max = a[i];
	}
	printf("%d\n", max - min);
	return 0;
}
