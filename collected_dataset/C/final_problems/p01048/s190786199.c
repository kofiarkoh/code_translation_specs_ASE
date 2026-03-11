// AOJ 1562: Divisor
// 2017.10.23 bal4u@uu

#include <stdio.h>

int ans[13] = { 0, 1, 2, 4, 6, 16, 12, 64, 24, 36, 48, 1024, 60 }; 

int main() 
{
	int n;

	scanf("%d", &n);
	printf("%d\n", ans[n]);
	return 0; 
}