#include<stdio.h>
int main()
{
	int k,i,j;
	long long n,ans=0,t;
	scanf("%lld%d",&n,&k);
	if(k==0)
	{
		ans=n*n;
		printf("%lld\n",ans);
		return 0;
	}
	for(i=k;i<=n;i++)
	{
         ans=ans+n-i;
	}
    for(i=k+1;i<=n;i++)
    {
      for(j=1;j<=i-k;j++)
      {
    	t=(n-(k+j-1))/i;
       if(t>0) ans=ans+t;
      }
	}
	printf("%lld\n",ans);
	return 0;
}