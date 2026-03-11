#include <stdio.h>

int main(void) {
	int min, max;
	scanf("%d", &min);
	scanf("%d", &max);

	for (int i = 0; i < 2; i++) {
		int t;
		scanf("%d", &t);
		if (t > max) max = t;
		if (t < min) min = t; 
	}
	printf("%d %d\n", min, max);
	return 0;
}
