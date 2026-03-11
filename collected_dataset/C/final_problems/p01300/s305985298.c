#include<stdio.h>
#include<string.h>
char S[80010];
int i,j,dp[80010][20],t,r,l;
int main()
{
	for(;scanf("%s",S),S[0]!='0';)
	{
		memset(dp,0,sizeof(dp));
		l=strlen(S);
		for(i=r=0;i<l;i++)
		{
			t=S[i]-'0';
			dp[i+1][t]=!!t;
			for(j=0;j<11;j++)
				if(dp[i][j])
					dp[i+1][(10*j+t)%11]+=dp[i][j];
			r+=dp[i+1][0];
		}
		printf("%d\n",r);
	}
	return 0;
}