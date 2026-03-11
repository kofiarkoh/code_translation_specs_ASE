#include<stdio.h>

int main()
{
	int n,k,i,ans=0;
	scanf("%d %d",&n,&k);
	if(k==0)printf("%d",n*n);
	else
	{
		for(i=k+1;i<=n;i++)
		{
			ans+=((n/i)*(i-k));
			if((n%i)>=k)ans+=((n%i)-k+1);
		}
		printf("%d",ans);
	}
}