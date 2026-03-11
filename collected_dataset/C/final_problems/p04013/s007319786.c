#include<stdio.h>
long long dp[51][51][3000] = {0};

int main(void)
{
  int n, a, i, j, k;
  int x[50];
  long long ans = 0;

  scanf("%d %d", &n, &a);

  for(i = 0; i < n; i++)
    scanf("%d", &x[i]);

  dp[0][0][0] = 1;
  
  for(i = 0; i <= n; i++)
    {
      for(j = 0; j <= n; j++)
	{
	  for(k = 0; k <= 50*50; k++)
	    {
	      if(dp[i][j][k])
		{
		  dp[i+1][j][k] += dp[i][j][k];
		  dp[i+1][j+1][k+x[i]] += dp[i][j][k];
		}
	    }
	}
    }

  for(i = 1; i <= n; i++)
    ans += dp[n][i][i*a];

  printf("%lld\n", ans);
  return 0;
}
