#include<stdio.h>
#include<stdlib.h>
#pragma warning(disable:4996)
#include<string.h>
#include<ctype.h>
int main()
{
	long long n,a[100000];
	int f[100000]={1};
	long long i,s,count=0;
	scanf("%lld",&n);
	for(i=1;i<=n;i++)
	{
		scanf("%lld",&a[i]);
		f[i]=1;
	}
	f[1]=0;
	s=a[1];
	count++;
	while(1)
	{
	if(f[s]!=0)
	{
		f[s]=0;
		s=a[s];
		count++;
	}
	if(s==2)break;
	if(f[s]==0)break;
		
	}
	if(s==2)printf("%lld",count);
	else printf("-1");
	return 0;
	
}