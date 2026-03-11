#include<stdio.h>
#include<string.h>
#define MX 100005
int N,D,X,i,j,K[15][15],dp[MX+1],dp2[MX+1];
#define max(a,b) (a)>(b)?(a):(b)
int np(int d,int w)
{
	int i,j,r=0;
	memset(dp2,0,sizeof(dp2));
	for(i=0;i<N;i++)
	{
		for(j=K[d][i];j<=w;j++)
		{
			dp2[j]=max(dp2[j],dp2[j-K[d][i]]+K[d+1][i]);
			r=max(r,dp2[j]+w-j);
		}
	}
	return r>MX?0:r;
}

int main()
{
	scanf("%d%d%d",&N,&D,&X);
	for(i=0;i<D;i++)
		for(j=0;j<N;j++)
			scanf("%d",&K[i][j]);
	dp[X]=1;
	for(i=0;i<D;i++)
	{
		for(j=MX;j>=0;j--)
			if(dp[j])
				dp[np(i,j)]=1;
	}
	for(i=MX;i>=0;i--)if(dp[i])break;
	printf("%d\n",i);
	return 0;
}