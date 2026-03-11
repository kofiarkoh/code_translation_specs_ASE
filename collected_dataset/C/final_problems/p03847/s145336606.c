#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#define MOD 1000000007
#define MAX 510000

long int FAC[MAX],FINV[MAX],INVE[MAX];
long int SUM(long int a,long int b){return (a%MOD+b%MOD+MOD)%MOD;}//a+b
long int PROD(long int a,long int b){return a%MOD*(b%MOD)%MOD;}//a*b
long int POW(long int a,long int b){//a^b
  long int c=1;
  for(a%=MOD;b;a=PROD(a,a),b>>=1)if(b&1)c=PROD(a,c);
  return c;
}
long int INV(long int a){return a<MAX?INVE[a]:POW(a,MOD-2);}//a^-1
long int QUOT(long int a,long int b){return PROD(a,INV(b));}//a/b
void init_inv(long int a){//make inv list
  INVE[1]=1;
  for(long int i=2;i<=a;i++)INVE[i]=SUM(0,-PROD(INVE[MOD%i],MOD/i));
}
void init_com(long int a){//make fac and facinv list
  FAC[0]=FAC[1]=1;
  FINV[0]=FINV[1]=1;
  init_inv(a);
  for(long int i=2;i<=a;i++){
    FAC[i]=PROD(FAC[i-1],i);
    FINV[i]=PROD(FINV[i-1],INV(i));
  }
}
long int COM(long int a,long int b){return PROD(PROD(FAC[a],FINV[a-b]),FINV[b]);}

int main(){
  long int n,i;
  long int dp[64][3];
  scanf("%ld",&n);
  dp[63][0]=1;dp[63][1]=dp[63][2]=0;
  for(i=62;i>=0;i--){
    if((n>>i)&1){
      dp[i][0]=dp[i+1][0];
      dp[i][1]=SUM(dp[i+1][0],dp[i+1][1]);
      dp[i][2]=SUM(PROD(dp[i+1][1],2),PROD(dp[i+1][2],3));
    }
    else{
      dp[i][0]=SUM(dp[i+1][0],dp[i+1][1]);
      dp[i][1]=dp[i+1][1];
      dp[i][2]=SUM(dp[i+1][1],PROD(dp[i+1][2],3));
    }

  }
  printf("%ld",SUM(SUM(dp[0][0],dp[0][1]),dp[0][2]));
}
