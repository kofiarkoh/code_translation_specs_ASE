#include<stdio.h>
int main()
{
	int n;
	scanf("%d", &n);
	if (n < 1000) {
		printf("ABC\n");
	}
	else {
		printf("ABD\n");
	}
	return 0;
}