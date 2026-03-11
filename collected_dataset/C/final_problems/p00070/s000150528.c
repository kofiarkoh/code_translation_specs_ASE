#include <stdio.h>

int num[10]={1,2,3,4,5,6,7,8,9,0}, dp[11][331];
#define exch(A,B) {int t; t=A, A=B, B=t;}

/* k¶Úðßé, sÉÍ»Ì_ÅÌað */
void solve(int k, int s){
     int i, j, ts;

     for(i=k; i<10; i++){
	  exch(num[i],num[k]);

	  /* n=kÅÌsðL^ */
	  ts=s;
	  ts+=num[k]*(k+1);
	  dp[k+1][ts]++;

	  if(k<9) solve(k+1,ts);

	  exch(num[i],num[k]);
     }
}

int main(int argc, char *argv[]){
     int n, s;

     solve(0,0);
     
     while(~scanf("%d %d",&n,&s))
	  printf("%d\n",(n<=10 && s<=330)?dp[n][s]:0);

     return 0;
}