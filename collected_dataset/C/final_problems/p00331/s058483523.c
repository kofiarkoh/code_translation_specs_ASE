#include<stdio.h>
int main()
{
	int h,r;
	scanf("%d%d",&h,&r);
	if(h*-1==r)
	{
		printf("0\n");
	}
	else if(h*-1>r)
	{
		printf("-1\n");
	}
		else if(h<=r)
	{
		printf("1\n");
	}
	return 0;
}