#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INF 1000000007

long dp[1005][1005];
char a[1005][1005];

int def(int H,int W);

int main(void){
  int H,W,i,j;
  char str[1100],*p;
  
  fgets(str,sizeof(str),stdin);
  p=strtok(str," \n");
  H=atoi(p);
  p=strtok(NULL," \n");
  W=atoi(p);
  
  for(i=0;i<H;i++){
    fgets(str,sizeof(str),stdin);
    strcpy(a[i],str);
  }
  
  def(H,W);
  
  printf("%ld\n",dp[H-1][W-1]);
  
  return 0;
}

int def(int H,int W){
  int i,j;
  
  dp[0][0]=1;
  
  for(i=0;i<H;i++){
    for(j=0;j<W;j++){
      if(a[i][j]=='.'){
        if(i>0 && j>0){
          dp[i][j]=dp[i-1][j]+dp[i][j-1];
          dp[i][j]%=INF;
        }else if(i==0 && j>0){
          dp[i][j]=dp[i][j-1];
          dp[i][j]%=INF;
        }else if(i>0 && j==0){
          dp[i][j]=dp[i-1][j];
          dp[i][j]%=INF;
        }
      }
    }
  }
  
  return 0;
}
