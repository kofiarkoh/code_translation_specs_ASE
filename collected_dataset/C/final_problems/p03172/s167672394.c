#include<stdio.h>
#include<math.h>
#include<stdlib.h>
 
long int N,K,A[101][100001],a[101];

long int dp(long int n,long int m)
{
  long int temp;
  long int s;
  
  if(A[n][m]!=-1)
  {
    return A[n][m];
  }
  else
  {
    
    temp=0;
    if(m>0)
    {
      temp+=dp(n,m-1);
    }
    if(n>1)
    {
      temp+=dp(n-1,m);
      if(m-1-a[n]>=0)
      {
        temp-=dp(n-1,m-1-a[n]);
      }
    }
      
    
  }
  

  temp=temp%1000000007;
  A[n][m]=temp;
  
  return temp;
  
}
 
int main()
{
  
  long int i,j;
  
  scanf("%ld %ld",&N,&K);
  

  for(i=1;i<N+1;i++)
  {
    scanf("%ld",&a[i]);
  }
  
  for(i=0;i<N+1;i++)
  {
    for(j=0;j<K+1;j++)
    {
      A[i][j]=-1;
    }
  }

  for(j=0;j<K+1;j++)
      {
        if(j<=a[1])
        {
          A[1][j]=1;
        }
        else
        {
          A[1][j]=0;
        }
      }
  

   printf("%ld",dp(N,K));

  
  return 0;
}
