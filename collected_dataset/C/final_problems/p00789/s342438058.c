#include<stdio.h>
#include<string.h>
#define MAX 300

int main(){
	int i,j,n;
	int dp[MAX];
	while(scanf("%d",&n)==1&&n){
		memset(dp,0,sizeof(dp));
		dp[0]=1;
		for(i=1;i*i<=n;i++){
			for(j=i*i;j<=n;j++){
				dp[j] += dp[j-i*i];
			}
		}
		//for(i=0;i<=n;i++)printf("%d ",dp[i]);
		printf("%d\n",dp[n]);
	}
	return 0;
}