#include<stdio.h>
#include<string.h>

int W,H,i,j;
char dp[71][71][71],g[2],tmp[71],res[71];

int cmp(char x[],char y[])
{
	int s=strlen(x),t=strlen(y);
	return s<t||(s==t&&strcmp(x,y)<0)?1:0;
}

int main()
{
	for(;scanf("%d%d\n",&W,&H),W;)
	{
		memset(dp,0,sizeof(dp));
		memset(res,0,sizeof(res));
		for(i=0;i<H;i++,getchar())
			for(j=0;j<W;j++)
			{
				tmp[0]=0;
				g[0]=getchar();
				if(g[0]<58)
				{
					if(i>0)
					{
						if(cmp(tmp,dp[i-1][j]))
							strcpy(tmp,dp[i-1][j]);
					}
					if(j>0)
					{
						if(cmp(tmp,dp[i][j-1]))
							strcpy(tmp,dp[i][j-1]);
					}
					sprintf(dp[i][j],"%s",strcat(tmp,g));
				}
				if(cmp(res,dp[i][j]))
					strcpy(res,dp[i][j]);
			}
		while(res[1]!=0&&res[0]=='0')
			memmove(res,res+1,strlen(res));
		puts(res);
	}
	return 0;
}