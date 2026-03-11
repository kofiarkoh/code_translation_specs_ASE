#include<stdio.h>

int main()
{
	int n,p,a[55],i;
	int num_1=0;
	long long int ans,m;
	scanf("%d%d",&n,&p);
	for(i=0;i<n;i++)
	{
		scanf("%d",&a[i]);	
		if(a[i]%2)	num_1++;
	}
	if(num_1==0)
	{
		if(p)
		{
			printf("0");
			return 0;
		}
		else
		{
			if(n%2)
			{
				ans=1<<(n/2);
				m=2<<(n/2);
				printf("%lld",m*ans);
				return 0;
			}
			else
			{
				ans=m=1<<(n/2);
				printf("%lld",ans*m);
				return 0;
			}
		}
	}
	ans=1<<(num_1-1);
	m=1<<(n-num_1);
	printf("%lld",ans*m);
	return 0;
}