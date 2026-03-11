#include<stdio.h>
#include <stdlib.h>
int main()
{
	int i, x = 7, K;
	scanf("%d", &K);
	if (K % 2 == 0 || K % 5 == 0) {
		printf("-1");
		exit(0);
	}
	for (i = 0; i < 3000000; i++) {
		x = (x * 10 + 7) % K;
		if (x == 0) {
			printf("%d", i+2);
			exit(0);
		}
	}
	printf("-1");
}
