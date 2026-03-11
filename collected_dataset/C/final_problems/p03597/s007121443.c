#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(void) {

	int N, A, x;
	scanf("%d%d", &N, &A);
	x = N * N - A;
	printf("%d\n", x);
	return 0;
}
