#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int main()
{
	int A, B;
	scanf("%d%d", &A, &B);

	int i, j, k;
	int num;
	for (i = A, num = 0; i <= B; i++) {
		if ((i % 10 == i / 10000) && (((i % 100) / 10) == ((i / 1000) % 10)))
			num += 1;
	}

	printf("%d\n", num);

	return 0;
}
