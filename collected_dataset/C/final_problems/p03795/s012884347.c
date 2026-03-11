#include <stdio.h>

int main(void)
{
	int n;
	int a;
	int w;
	scanf("%d",&n);
	a=n*800;
	w=a-n/15*200;
	printf("%d\n",w);
	return 0;
}