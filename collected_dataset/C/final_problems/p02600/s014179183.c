#include <stdio.h>

int main(void)
{
	int x;
	scanf("%d", &x);
	
	if(x >= 400 && x <= 599)
		printf("8");
	if(x >= 600 && x <= 799)
		printf("7");
	if(x >= 800 && x <= 999)
		printf("6");
	if(x >= 1000 && x <= 1199)
		printf("5");
	if(x >= 1200 && x <= 1399)
		printf("4");
	if(x >= 1400 && x <= 1599)
		printf("3");
	if(x >= 1600 && x <= 1799)
		printf("2");
	if(x >= 1800 && x <= 1999)
		printf("1");
	return (0);
}
