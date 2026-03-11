#include <stdio.h>

int main()
{
	int a, b, c, d, e, k;
	int i, j;
	int count=0;

	int antena[5];

	scanf("%d %d %d %d %d %d", &antena[0], &antena[1], &antena[2], &antena[3], &antena[4], &k);

	for (i = 4; i >= 0; i--) {
		for (j = 0; j <= i; j++) {
			if (antena[i] - antena[j] > k) {
				count++;
			}

		}
	}

	if (count >= 1) {
		printf(":(\n");
	}
	else {
		printf("Yay!\n");
	}

	return 0;
	
}
