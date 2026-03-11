#include<stdio.h>
#include<math.h>
int main()
{
	long long i,x,y,sum=0;
	scanf("%lld%lld",&x,&y);
	while(x<=y)
	{
		x=x*2;
		sum++;
	}
	printf("%lld\n",sum);
}