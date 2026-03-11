#include<stdio.h>
#define M(a,b)a<b?b:a
int w[100],v[100],n,W,i,j,dp[10005];
int main()
{
	scanf("%d%d",&n,&W);
	for(i=0;i<n;i++)
		scanf("%d%d",&v[i],&w[i]);
	for(i=0;i<n;i++)
		for(j=W;j>=0;j--)
			if(j>=w[i])
				dp[j]=M(dp[j],dp[j-w[i]]+v[i]);
	printf("%d\n",dp[W]);
	return 0;
}