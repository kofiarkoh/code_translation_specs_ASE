#include <stdio.h>

#define REP(i,a,n) for(int i=(a); i<(int)(n); i++)
#define rep(i,n) REP(i,0,n)
#define DEB 0

//answer_max:2192
unsigned short int dp[5][32770];

int main(){
  unsigned short int i,j,k;
  int n;
  for(i=1; i*i<(1<<15); i++){
    dp[1][i*i] = 1;
    for(j=i*i+1; j<(1<<15); j++){
      for(k=1; k<4; k++){
	dp[k+1][j] += dp[k][j-i*i];
      }
    }
  }
  while(scanf("%d",&n),n){
    printf("%d\n",dp[1][n]+dp[2][n]+dp[3][n]+dp[4][n]);
  }
  return 0;
}