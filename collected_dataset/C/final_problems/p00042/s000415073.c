#include<stdio.h>
#include<math.h>
#include <stdlib.h>
#include<string.h>
#include<limits.h>
#include<time.h>

int  main(){
	int W,N,i,j,k,max_coin=0,weg=0,count=0;
	int wg[10000]={0},coin[10000]={0};
	int dp[1000][1000];
	
	while(1){
	scanf("%d",&W);
	if(W==0)break;
	scanf("%d",&N);
	
	for(i=0;i<=N;i++){
		for(j=0;j<=W;j++)dp[i][j]=-1;
	}
	
	for(i=1;i<=N;i++)scanf("%d,%d",&coin[i],&wg[i]);
	
	dp[0][0]=0;
	
	for(i=1;i<=N;i++){
		dp[i][0]=0;
		if(dp[i][wg[i]]<coin[i])dp[i][wg[i]]=coin[i];
		for(j=i;j<=N;j++)dp[j][wg[i]]=dp[i][wg[i]];
		
		for(j=1;j<=W;j++){
			if(dp[i-1][j]>0 && dp[i][j+wg[i]]<(coin[i]+dp[i-1][j])){
				dp[i][j+wg[i]]=coin[i]+dp[i-1][j];
				for(k=i;k<=N;k++)dp[k][j+wg[i]]=dp[i][j+wg[i]];
			}
		}

	
	}
	
	for(i=0;i<=N;i++){
		for(j=0;j<=W;j++){
			if(max_coin<dp[i][j]){
				max_coin=dp[i][j];
				weg=j;
			}
			else if(max_coin==dp[i][j]){
				if(weg>=j)weg=j;
			}
		}
	}
	count++;
	printf("Case %d:\n%d\n%d\n",count,max_coin,weg);
		max_coin=0;
		weg=0;
	}
return 0;
}