#include <stdio.h>
#include <string.h>

int sqr(int x)
{
	return x*x;
}

int main()
{
	int r;
	
	scanf("%d", &r);
	
	printf("%d", sqr(r));
	
return 0;
}