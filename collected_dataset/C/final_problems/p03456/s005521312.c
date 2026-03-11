#include <stdio.h>

int main(void) {
	int a, b;
	int i;
	int k=-1;
	scanf("%d%d", &a, &b);
	if (a >= 10 && b < 10) {
		i = 10 * a+b;
	}
	else if (a >= 10 && b >= 10) {
		i = 100 * a + b;
	}
	else if (a < 10 && b < 10) {
		i = 10 * a + b;
	}
	else if (a < 10 && b >= 10) {
		i = 100 * a + b;
	}

	for (int j = 1; j < 400; j++) {
		if (i == j * j) {
			k = j;
		}
		

	}

	if (k == -1) {
		printf("No");
	}
	else {
		printf("Yes");
	}


	return 0;

}