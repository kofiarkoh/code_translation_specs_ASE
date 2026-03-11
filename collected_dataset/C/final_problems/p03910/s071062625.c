#include <stdio.h>
#include <math.h>

int main(void)
{
	int i, j, k, l,m;
	scanf("%d", &i);
	j = 0;

	for (m = 0; m <= 10000; m++) {

		if (j * (j + 1) / 2 >= i) {
			k = -i + j * (j + 1) / 2;
			for (l = 1; l <= j; l++) {
				if (k != l) {
					printf("%d\n", l);
				}
			}break;
		}
		else {
			j++;
		}
	}


	return 0;
}