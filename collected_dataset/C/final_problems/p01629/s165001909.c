#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int N,D,p[51][51],dp[51][51][53],i,j,k,r;//50-cost 51-change

int main()
{
	scanf("%d%d",&N,&D);
	for(i=0;i<N;i++)
		for(j=0;j<D;j++)
			scanf("%d",&p[i][j]);
	for(i=0;i<=D;i++)
		for(j=0;j<=N;j++)
		{
			dp[i][j][50]=10000000;
			dp[i][j][51]=0;
		}
	dp[0][0][50]=0;
	for(i=0;i<D;i++)
		for(j=0;j<N;j++)
		{
			for(k=0;k<N;k++)
			{
				if(dp[i+1][k][50]>dp[i][j][50]+p[k][i]||
					(dp[i+1][k][50]==dp[i][j][50]+p[k][i]&&dp[i+1][k][51]>dp[i][j][51]+(j==k?0:1))||
					(dp[i+1][k][50]==dp[i][j][50]+p[k][i]&&dp[i+1][k][51]==dp[i][j][51]+(j==k?0:1)&&memcmp(dp[i+1][k],dp[i][j],sizeof(int[50])>0)))
				{
					memcpy(dp[i+1][k],dp[i][j],sizeof(dp[i][j]));
					dp[i+1][k][i]=k;dp[i+1][k][50]+=p[k][i];
					if(i!=0&&k!=j)
						dp[i+1][k][51]++;
				}
			}
		}

	for(i=r=0;i<N;i++)
	{
		if(dp[D][r][50]>=dp[D][i][50])
		{
			if(dp[D][r][50]>dp[D][i][50])
				r=i;
			if(dp[D][r][51]>=dp[D][i][51])
			{
				if(dp[D][r][51]>dp[D][i][51]||memcmp(dp[D][r],dp[D][i],sizeof(int[50])>0))
					r=i;
			}
		}
	}
	printf("%d %d\n",dp[D][r][50],dp[D][r][51]);
	for(i=0;i<D;i++)
		printf("%d\n",dp[D][r][i]+1);
	return 0;
}