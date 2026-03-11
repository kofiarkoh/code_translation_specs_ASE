#include<stdio.h>
#define MAX 2147483647

int dp[100][100],num[101];

int rensa(int);

int main(){
  int n,i,j;
  scanf("%d",&n);
  for(i=0;i<n;i++) scanf("%d%d",num+i,num+i+1);
  printf("%d\n",rensa(n));  
  return 0;
}

int rensa(int n){
  int i,j,k,l,min;
  for(i=0;i<n;i++) dp[i][i]=0;
  
  for(l=1;l<n;l++){
    for(i=0;i<n-l;i++){
      j=i+l;
      dp[i][j]=MAX;
      for(k=i;k<j;k++){
        min=dp[i][k]+dp[k+1][j]+num[i]*num[k+1]*num[j+1];
        if(dp[i][j]>min) dp[i][j]=min;
      }
    }
  }
  return dp[0][n-1];
}
