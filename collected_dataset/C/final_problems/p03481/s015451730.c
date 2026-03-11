#include<stdio.h>
int main()
{
	long long x,y,i,sum;
	scanf("%lld %lld",&x,&y);
	sum=0;
	for(i=x;i<=y;i=i+i)
	{
		if(i<=y)sum++;
	};
	printf("%lld",sum);
	return 0;			
}