#include<stdio.h>
#include<string.h>
int N,M,G[100][16],i,j,k,L[100],S,E,R;
long long dp[1<<18][2];
int main()
{
	for(;scanf("%d",&N),N;)
	{
		memset(G,0,sizeof(G));
		for(i=0;i<N;i++)
		{
			
			scanf("%d%d",&M,&L[i]);
			for(j=0;j<M;j++)
			{
				scanf("%d%d",&S,&E);
				S-=6;E-=6;
				G[i][j]=((1<<(E+1))-1)-((1<<(S+1))-1);
			}
		}
		memset(dp,-1,sizeof(dp));
		dp[0][0]=0;
		for(i=R=0;i<(1<<18);i++)
		{
			if(dp[i][0]==-1)continue;
			for(j=0;j<N;j++)
				for(k=0;G[j][k];k++)
				{
					if((i&G[j][k])==0&&((dp[i][1]>>j)&1)&&dp[i|G[j][k]][0]<dp[i][0]+L[j])
					{
						dp[i|G[j][k]][0]=dp[i][0]+L[j];
						dp[i|G[j][k]][1]=(dp[i][1]^(1<<j));
						R=R<dp[i][0]+L[j]?dp[i][0]+L[j]:R;
					}
				}
		}
		printf("%d\n",R);
	}
	return 0;
}