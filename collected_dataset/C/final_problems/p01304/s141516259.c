#include<stdio.h>
#include<string.h>
int c,n,i,j,f[16][16],dp[16][16],x,y,fx,fy,tx,ty;
int main()
{
	scanf("%d",&c);
	for(;c--;)
	{
		memset(f,0,sizeof(f));
		memset(dp,0,sizeof(dp));
		scanf("%d%d%d",&x,&y,&n);
		for(i=0;i<n;i++,f[fy][fx]=f[ty][tx]=1)
			scanf("%d%d%d%d",&fx,&fy,&tx,&ty);
		dp[0][0]=1;
		for(i=1;i<=x;i++)
			dp[0][i]=f[0][i-1]&f[0][i]?0:dp[0][i-1];
		for(i=1;i<=y;i++)
			dp[i][0]=f[i-1][0]&f[i][0]?0:dp[i-1][0];
		for(i=1;i<=y;i++)
			for(j=1;j<=x;j++)
			{
				dp[i][j]+=f[i][j-1]&f[i][j]?0:dp[i][j-1];
				dp[i][j]+=f[i-1][j]&f[i][j]?0:dp[i-1][j];
			}
		if(!dp[y][x])
			puts("Miserable Hokusai!");
		else
			printf("%d\n",dp[y][x]);
	}
	return 0;
}