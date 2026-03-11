#include<stdio.h>
#define SIZE 100
int main()
{
  int n,m;
  int i,j;
  int s[2][SIZE];
  int ans[2];
  int sum[3];
  while(1)
    {
      scanf("%d %d",&n,&m);
      if((n==0)&&(m==0)) break;
      sum[0]=0;
      for(i=0;i<n;i++)
	{
	  scanf("%d",&s[0][i]);
	  sum[0]+=s[0][i];
	}
      sum[1]=0;
      for(i=0;i<m;i++)
	{
	  scanf("%d",&s[1][i]);
	  sum[1]+=s[1][i];
	}
      ans[0]=ans[1]=-1;
      sum[2]=201;
      for(i=0;i<n;i++)
	{
	  for(j=0;j<m;j++)
	    {
	      if((sum[0]-s[0][i]+s[1][j]==sum[1]-s[1][j]+s[0][i])&&(s[0][i]+s[1][j]<sum[2]))
		{
		  ans[0]=i;
		  ans[1]=j;
		  sum[2]=s[0][i]+s[1][j];
		}
	    }
	}
      if(ans[0]!=-1) printf("%d %d\n",s[0][ans[0]],s[1][ans[1]]);
      else printf("-1\n");
    }
  return 0;
}