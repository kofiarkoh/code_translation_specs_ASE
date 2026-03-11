#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int median(int a, int b, int c) {
	if ((a>=b && b>=c) || (c>=b && b>=a)) {
		return b;
	} else if ((c>=a && a>=b) || (b>=a && a>=c)) {
		return a;
	} else return c;
}

int intcmp(const void *a, const void *b) {
	return *(int *)a - *(int *)b;
}

int main(void) {
	int a[200020], flag=0, i, j, N;
	scanf("%d", &N);
	for (i=0;i<2*N-1;i++) scanf("%d", &a[i]);

	for (j=0;j<2*N-1;j++) if (a[j]>a[j+1]) flag=0;
	if (flag) i=N;
	else for (i=N-1;i>0;i--) {
		for (j=0;j<2*i-1;j++) {
			a[j] = median(a[j], a[j+1], a[j+2]);
		}
		flag = 1;
		for (j=0;j<2*i-1;j++) if (a[j]>a[j+1]) flag=0;
		if (flag) break;
	}
	qsort(a, 2*i-1, sizeof(int), intcmp);

	printf("%d\n", a[i-1]);

	return 0;
}
