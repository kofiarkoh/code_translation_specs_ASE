#include <stdio.h>
#include <stdlib.h>

int main(void) {
	int n = 0, i = 0, xsm = 0;
	int num[100];

	if(scanf("%d", &n)==NULL){
		return 0;
	}

	for (i = 0; i < (2 * n); i++) {
		if(scanf("%d\n", &num[i])==NULL){
			return 0;
		}

	}

	int j, k, temp;

	for (j = 1; j < (2 * n); j++) {
		temp = num[j];
		for (k = j; k > 0 && num[k - 1] > temp; k--) {
			num[k] = num[k - 1];
		}
		num[k] = temp;
	}

	for (i = 0; i < (2 * n); i++) {
		xsm += num[i];
		i++;
	}
	printf("%d\n", xsm);
	return 0;
}
