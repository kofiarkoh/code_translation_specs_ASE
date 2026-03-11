#include <stdio.h>

int main(void) {
	long int a;
	long int b;
	long int x;
	scanf("%ld %ld %ld", &a, &b, &x);
	long int count = 0;
	count = (b - a) / x;
	if (a % x == 0 || b % x == 0) {
		count++;
	}
	printf("%ld", count);
	return 0;
}