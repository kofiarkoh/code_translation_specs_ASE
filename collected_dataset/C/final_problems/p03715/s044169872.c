#include<stdio.h>
int main(void)
{
	long long a,b,t;
	scanf("%lld%lld",&a,&b);
	if(a<b)
	{
		t=a;
		a=b;
		b=t;
	}
	if(a%3==0||b%3==0)
	printf("0\n");
	else
	{
		if(a%2==1&&b%2==1)
		{
			t=(b/2*a)-(b/2+1)*(a/2);
			printf("%lld\n",t);
		}
		else if(a%2==0&&b%2==0)
		{
			b=b/2;
			printf("%lld\n",b);	
		}
		else if(a%2==0&&b%2==1)
		{
			a=a/2;
			printf("%lld\n",a);
		}
		else if(a%2==1&&b%2==0)
		{
			b=b/2;
			printf("%lld\n",b);
		}
	}
	return 0;
}