#include<stdio.h>
#include<string.h>

int main(void)
{
	int i,j,n,dp[1001];
	char s1[1000],s2[1000];
	
	scanf("%s%s",s1,s2);
	
	j=0;
	dp[0]=0;
	for(i=0;i<=strlen(s1)-1;i++)
	{
		if(s1[i]==s2[j])
		{
			j++;
			dp[i+1]=dp[i]+1;
		}
	    else
		{
		    dp[i+1]=dp[i];
	    }
	}
	if(strlen(s1)>=strlen(s2))
	{
	    n=strlen(s1)-dp[strlen(s1)];
	}
    else
	{
	    n=strlen(s2)-dp[strlen(s1)]+1;
	}
	printf("%d\n",n);
	return 0;
}