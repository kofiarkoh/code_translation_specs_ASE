#include<stdio.h>
#include<string.h>

char A[105],B[105],C[105];
int dp[105][2],d,c,i,j,a,b;
const int M=1000000007;
int main()
{
	for(;scanf("%s%s%s",A,B,C),A[0]!='0';)
	{
		memset(dp,0,sizeof(dp));
		dp[strlen(A)][0]=1;
		for(d=strlen(A)-1;d>=0;d--)
			for(c=0;c<2;c++)
			{
				if(A[d]=='?')
				{
					for(a=d==0?1:0;a<=9;a++)
						if(B[d]=='?')
						{
							for(b=d==0?1:0;b<=9;b++)
							{
								if(C[d]=='?'||C[d]-'0'==(a+b+c)%10)
									dp[d][a+b+c>9?1:0]=(dp[d][a+b+c>9?1:0]+dp[d+1][c])%M;
							}
						}
						else
						{
							b=B[d]-'0';
							if(C[d]=='?'||C[d]-'0'==(a+b+c)%10)
								dp[d][a+b+c>9?1:0]=(dp[d][a+b+c>9?1:0]+dp[d+1][c])%M;
						}
				}
				else
				{
					a=A[d]-'0';
					if(B[d]=='?')
					{
						for(b=d==0?1:0;b<=9;b++)
						{
							if(C[d]=='?'||C[d]-'0'==(a+b+c)%10)
								dp[d][a+b+c>9?1:0]=(dp[d][a+b+c>9?1:0]+dp[d+1][c])%M;
						}
					}
					else
					{
						b=B[d]-'0';
						if(C[d]=='?'||C[d]-'0'==(a+b+c)%10)
							dp[d][a+b+c>9?1:0]=(dp[d][a+b+c>9?1:0]+dp[d+1][c])%M;
					}
				}
			}
		printf("%d\n",dp[0][0]);
	}
	return 0;
}