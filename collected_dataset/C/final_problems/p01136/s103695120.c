#include<stdio.h>

int main()
{
  int n,i,j,sum,k;
  int SC[50][31];
  while(1){
    scanf("%d",&n);
    if(n==0)break;
    for(i=0;i<n;i++){
      scanf("%d",&SC[i][0]);
      for(j=1;j<SC[i][0]+1;j++){
	scanf("%d",&SC[i][j]);
      }
    }
    for(k=1;k<30;k++){
      sum=0;
      for(i=0;i<n;i++){
	for(j=1;j<SC[i][0]+1;j++){
	  if(SC[i][j]<=k){
	    sum++;
	    break;
	  }
	}
      }
      if(sum==n){
	printf("%d\n",k);
	break;
      }
    }
    if(k==30)printf("-1\n");
  }
    return 0;
}