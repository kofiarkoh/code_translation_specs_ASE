#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>

int main(void) {
	long long a, b_100, output;
	long double b;

	scanf("%lld", &a);
	scanf("%Lf", &b);

	b_100 = b * 100;

	output = a * (long long)b_100;

	output /= 100;

	printf("%lld", output);

	return 0;
}