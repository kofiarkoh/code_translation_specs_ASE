#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main()
{
  int N,i,j,k;
  int max;
  scanf("%d",&N);
  int S[N+1],L[N+1],P[N+1];
  S[0]=0;
  L[0]=0;
  P[0]=-1;
  for(i=1;i<=N;i++)
    scanf("%d",&S[i]);
  
  for(i=1;i<=N;i++){
    k=0;
    for(j=0;j<=i-1;j++)
      if(S[j]<S[i] && L[j]>L[k])
	k=j;
    L[i]=L[k]+1;
    P[i]=k;
    
  }
  max=0;
  for(i=1;i<=N;i++) 
    if(max<L[i])
      max=L[i];
  printf("%d\n",max);
  return 0;
}