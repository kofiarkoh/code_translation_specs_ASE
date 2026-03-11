#include<stdio.h>
#include<string.h>
#define min(a,b) (a<=b?a:b)
#define max(a,b) (a<=b?b:a)
#define lim(a) (min(max(a,0),255))

int N,M,S[17],I[20005],i,j,k,R,e;
int dp[20005][256];
int main()
{
	for(;scanf("%d%d",&N,&M),N;)
	{
		for(i=0;i<M;i++)scanf("%d",&S[i]);
		for(i=1;i<=N;i++)scanf("%d",&I[i]);
		memset(dp,-1,sizeof(dp));
		dp[0][128]=0;
		for(i=0;i<=N;i++)
			for(j=0;j<=255;j++)
			{
				if(dp[i][j]!=-1)
					for(k=0;k<M;k++)
					{
						e=lim(j+S[k])-I[i+1];
						if(dp[i+1][lim(j+S[k])]==-1||dp[i+1][lim(j+S[k])]>dp[i][j]+e*e)
							dp[i+1][lim(j+S[k])]=dp[i][j]+e*e;
					}
			}
		for(i=0,R=-1;i<=255;i++)
			if(dp[N][i]!=-1&&(R==-1||R>dp[N][i]))
				R=dp[N][i];
		printf("%d\n",R);
	}
	return 0;
}