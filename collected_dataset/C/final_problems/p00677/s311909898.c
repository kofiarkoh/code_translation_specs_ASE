#include<stdio.h>
#include<string.h>
int S,D,M,K[105],W[105][55],P[105][55],F,N[51][301],i,j,k,l,w,p,dp[305];
int main()
{
	for(;~scanf("%d%d%d",&S,&D,&M);)
	{
		memset(dp,-1,sizeof(dp));
		memset(N,0,sizeof(N));
		dp[0]=0;
		for(i=0;i<S;i++)
			for(scanf("%d",&K[i]),j=0;j<K[i];j++)
				scanf("%d%d",&W[i][j],&P[i][j]);
		for(i=0;i<D;i++)
		{
			scanf("%d",&F);
			for(j=0;j<K[F];j++)
				N[W[F][j]][P[F][j]]++;
		}
		for(i=0;i<51;i++)
			for(j=0;j<=M;j++)
				if(N[i][j]>0)
					for(k=M;k>=0;k--)
						if(dp[k]>=0)
							for(l=1;l<=N[i][j]&&k+j*l<=M;l++)
							{
								if(j==0)dp[k+j*l]=dp[k+j*l]<dp[k]+i?dp[k]+i:dp[k+j*l];
								else dp[k+j*l]=dp[k+j*l]<dp[k]+i*l?dp[k]+i*l:dp[k+j*l];
							}
		for(i=w=p=0;i<=M;i++)
			if(w<dp[i])
			{
				w=dp[i];
				p=i;
			}
		printf("%d %d\n",w,p);
	}
	return 0;
}