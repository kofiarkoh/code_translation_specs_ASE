#include <stdio.h>

int main(void)
{
	int x1,x2,sum;
	scanf("%d %d",&x1,&x2);
	if (x1>x2)printf("%d\n",x1-x2);
	else printf("%d\n",x2-x1);
	return 0;
}
