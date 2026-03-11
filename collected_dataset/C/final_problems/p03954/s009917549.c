#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int intcmp(const void *a, const void *b) {
	return *(int *)a - *(int *)b;
}

int main(void) {
	int a[200020], i, j, N;
	scanf("%d", &N);
	for (i=0;i<2*N-1;i++) scanf("%d", &a[i]);

	qsort(a, 2*N-1, sizeof(int), intcmp);

	printf("%d\n", a[N-1]);

	return 0;
}
