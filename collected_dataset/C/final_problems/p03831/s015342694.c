#include<stdio.h>
int main()
{
	int i;
	int s[100000];
	int n,a,b;
	scanf("%d %d %d",&n,&a,&b);
	for(i = 0;i<n;i++)
	{
		scanf("%d",&s[i]);
	}
	long long int sum = 0;
	for(i = 1;i<n;i++)
	{
		if((s[i] - s[i-1]) * a > b)
		{
			sum += b;
		}
		else
		{
			sum += (s[i] - s[i-1])*a;
		}
	}
	printf("%lld",sum);
	return 0;
}