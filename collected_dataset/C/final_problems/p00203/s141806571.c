#include<stdio.h>
#include<string.h>
int x,y,dp[20][15],m[15][15],i,j,k,t,r;
int main()
{
	for(;scanf("%d%d",&x,&y),x;)
	{
		memset(dp,0,sizeof(dp));
		for(i=0;i<y;i++)
			for(j=0;j<x;j++)
				scanf("%d",&m[i][j]);
		for(i=0;i<x;i++)
			if(!m[0][i])
				dp[0][i]=1;
		for(i=0;i<y-1;i++)
			for(j=0;j<x;j++)
			{
				if(m[i][j]==0)
				{
					dp[i+1][j]+=dp[i][j];
					if(j>0&&m[i+1][j-1]==0)
						dp[i+1][j-1]+=dp[i][j];
					if(j<y-1&&m[i+1][j+1]==0)
						dp[i+1][j+1]+=dp[i][j];
				}
				else if(m[i][j]==1)
					dp[i][j]=0;
				else
					dp[i+2][j]+=dp[i][j];
			}
		for(i=r=0;i<x;i++)
			r+=dp[y-1][i]+dp[y][i]+dp[y+1][i];
		printf("%d\n",r);
	}
	return 0;
}